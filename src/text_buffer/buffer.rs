use super::{
    caret::{Caret, Carets},
    file::{Indentation, LineFeed},
    position::{Absolute, Column, Line, Point, Position, Relative},
    rope_utils, SelectionLineRange,
};
use druid::Data;
use ropey::{Rope, RopeSlice};
use std::{cell::Cell, ops::{Bound, Range, RangeBounds}};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Buffer {
    rope: Rope,
    carets: Carets,
    pub(super) tabsize: usize,
    uuid: Uuid,
    max_visible_line_grapheme_len: Cell<usize>,
}

impl Data for Buffer {
    fn same(&self, other: &Self) -> bool {
        self.uuid == other.uuid && self.carets == other.carets
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new(super::file::Indentation::default().visible_len())
    }
}

impl Buffer {
    pub fn new(tabsize: usize) -> Self {
        Self {
            rope: Rope::new(),
            carets: Carets::new(),
            uuid: Uuid::new_v4(),
            tabsize,
            max_visible_line_grapheme_len: Cell::new(0),
        }
    }

    pub fn from_rope(rope: Rope, tabsize: usize) -> Self {
        Self {
            rope,
            carets: Carets::new(),
            uuid: Uuid::new_v4(),
            tabsize,
            max_visible_line_grapheme_len: Cell::new(0),
        }
    }

    /// Construct a string with tab replaced as space
    pub fn displayable_line(
        &self,
        line: Line,
        out: &mut String,
        rel_to_byte: &mut Vec<Relative>,
        byte_to_rel: &mut Vec<Relative>,
    ) {
        line.displayable_string(&self, self.tabsize, out, rel_to_byte, byte_to_rel);
        let l = line.grapheme_len(self).index.max(self.max_visible_line_grapheme_len.get());
        self.max_visible_line_grapheme_len.set(l);
    }

    pub fn max_visible_line_grapheme_len(&self) -> usize {
        self.max_visible_line_grapheme_len.get()
    }

    pub fn carets_on_line(&self, line: Line) -> impl Iterator<Item = &Caret> {
        self.carets.iter().filter(move |c| c.line() == line)
    }

    pub fn selection_on_line(&self, line_idx: usize, ranges: &mut Vec<SelectionLineRange>) {
        ranges.clear();
        for r in self.carets.iter().filter_map(move |c| {
            if !c.selection_is_empty() {
                let r = c.range();
                match (
                    self.rope.byte_to_line(r.start.into()),
                    self.rope.byte_to_line(r.end.into()),
                ) {
                    (s, e) if s == e && s == line_idx => Some(SelectionLineRange::Range(
                        self.position_to_point(r.start).relative.index..self.position_to_point(r.end).relative.index,
                    )),
                    (s, _) if s == line_idx => Some(SelectionLineRange::RangeFrom(
                        self.position_to_point(r.start).relative.index..,
                    )),
                    (_, e) if e == line_idx => Some(SelectionLineRange::RangeTo(
                        ..self.position_to_point(r.end).relative.index,
                    )),
                    (s, e) if line_idx < e && line_idx > s => Some(SelectionLineRange::RangeFull),
                    _ => None,
                }
            } else {
                None
            }
        }) {
            ranges.push(r);
        }
    }

    pub fn position_to_absolute<P>(&self, pos: P) -> Absolute
    where
        P: Position,
    {
        pos.absolute(&self)
    }
    pub fn position_to_point<P>(&self, pos: P) -> Point
    where
        P: Position,
    {
        pos.point(&self)
    }

    pub fn len(&self) -> Absolute {
        self.rope.len_bytes().into()
    }

    pub fn len_lines(&self) -> usize {
        self.rope.len_lines()
    }

    pub fn point<C, L>(&self, col: C, line: L) -> Point
    where
        C: Into<Column>,
        L: Into<Line>,
    {
        Point::new(col.into(), line.into(), &self)
    }

    pub fn line(&self, line_index: usize) -> Line {
        Line::from(line_index)
    }

    pub(super) fn line_slice<L>(&self, line: L) -> RopeSlice
    where
        L: Into<Line>,
    {
        self.rope.line(line.into().index)
    }

    pub(super) fn absolute_to_line(&self, a: Absolute) -> Line {
        self.rope.byte_to_line(a.index).into()
    }

    pub(super) fn line_to_absolute<L>(&self, line: L) -> Absolute
    where
        L: Into<Line>,
    {
        self.rope.line_to_byte(line.into().index).into()
    }

    pub fn char<P>(&self, pos: P) -> char
    where
        P: Position,
    {
        let a = pos.absolute(&self);
        self.rope.char(self.rope.byte_to_char(a.index))
    }

    pub fn backward(&mut self, expand_selection: bool, word_boundary: bool) {
        let b = self.clone();
        for s in &mut self.carets.iter_mut() {
            // TODO: Found a way to not clone, even if it's cheap
            s.move_backward(expand_selection, word_boundary, &b);
        }

        self.carets.merge();
    }

    pub fn forward(&mut self, expand_selection: bool, word_boundary: bool) {
        let b = self.clone();
        for s in &mut self.carets.iter_mut() {
            s.move_forward(expand_selection, word_boundary, &b);
        }

        self.carets.merge();
    }

    pub fn up(&mut self, expand_selection: bool) {
        let b = self.clone();
        for s in &mut self.carets.iter_mut() {
            s.move_up(expand_selection, &b);
        }

        self.carets.merge();
    }
    pub fn down(&mut self, expand_selection: bool) {
        let b = self.clone();
        for s in &mut self.carets.iter_mut() {
            s.move_down(expand_selection, &b);
        }

        self.carets.merge();
    }
    pub fn duplicate_down(&mut self) {
        self.carets.sort_unstable();

        if let Some(c) = self.carets.last().and_then(|c| c.duplicate_down(&self)) {
            self.carets.push(c);
        }
        self.carets.merge();
    }

    pub fn duplicate_up(&mut self) {
        self.carets.sort_unstable();

        if let Some(c) = self.carets.first().and_then(|c| c.duplicate_up(&self)) {
            self.carets.push(c);
        }
        self.carets.merge();
    }

    pub fn have_selection(&self) -> bool {
        self.carets.iter().any(|c| !c.selection_is_empty())
    }

    pub fn end(&mut self, expand_selection: bool) {
        let b = self.clone();
        for s in &mut self.carets.iter_mut() {
            s.move_end(expand_selection, &b);
        }

        self.carets.merge();
    }

    pub fn home(&mut self, expand_selection: bool) {
        let b = self.clone();
        for s in &mut self.carets.iter_mut() {
            s.move_home(expand_selection, &b);
        }
        self.carets.merge();
    }

    pub fn insert(&mut self, text: &str) {
        for i in 0..self.carets.len() {
            let r = self.carets[i].range();
            self.edit(&r, text);
            let b = self.clone();
            self.carets[i].set_index(r.start + text.len(), true, true, &b);
        }
        self.carets.merge();
    }

    pub fn backspace(&mut self) -> bool {
        let mut did_nothing = true;
        for i in 0..self.carets.len() {
            if !self.carets[i].selection_is_empty() {
                // delete all the selection
                let r = self.carets[i].range();
                self.edit(&r, "");
                let b = self.clone();
                self.carets[i].set_index(r.start, true, true, &b);

                did_nothing = false;
            } else if self.carets[i].index > 0.into() {
                // delete the preceding grapheme
                let r = rope_utils::prev_grapheme_boundary(&self.rope.slice(..), self.carets[i].index).into()
                    ..self.carets[i].index;
                self.edit(&r, "");
                let b = self.clone();
                self.carets[i].set_index(r.start, true, true, &b);

                did_nothing = false;
            } else {
                continue;
            }
        }
        if !did_nothing {
            self.carets.merge();
            return true;
        }
        false
    }

    pub fn delete(&mut self) -> bool {
        let mut did_nothing = true;
        for i in 0..self.carets.len() {
            if !self.carets[i].selection_is_empty() {
                let r = self.carets[i].range();
                self.edit(&r, "");
                let b = self.clone();
                self.carets[i].set_index(r.start, true, true, &b);

                did_nothing = false;
            } else if self.carets[i].index < self.rope.len_bytes().into() {
                let r = self.carets[i].index
                    ..rope_utils::next_grapheme_boundary(&self.rope.slice(..), self.carets[i].index).into();
                self.edit(&r, "");
                let b = self.clone();
                self.carets[i].set_index(r.start, true, true, &b);

                did_nothing = false;
            } else {
                continue;
            }
        }
        if !did_nothing {
            self.carets.merge();
            return true;
        }
        false
    }

    pub fn tab(&mut self, indentation: Indentation) {
        for i in 0..self.carets.len() {
            if let Some(line_range) = self.carets[i].selected_lines_range(&self) {
                // TODO: Find a better way to iterate over line of a selection
                for line_idx in line_range.start().index..line_range.end().index + 1 {
                    let line_start: Absolute = self.rope.line_to_byte(line_idx).into();
                    let r = line_start..line_start;
                    let text = match indentation {
                        Indentation::Space(n) => " ".repeat(n).to_owned(),
                        Indentation::Tab(_) => "\t".to_owned(),
                    };
                    self.edit(&r, &text);
                }
            } else {
                let r = self.carets[i].range();
                let text = match indentation {
                    Indentation::Space(n) => {
                        let start: usize = self.carets[i].col().into();
                        let nb_space = n - start % n;
                        " ".repeat(nb_space).to_owned()
                    }
                    Indentation::Tab(_) => "\t".to_owned(),
                };
                self.edit(&r, &text);
                let b = self.clone();
                self.carets[i].set_index(r.start + Relative::from(text.len()), true, true, &b);
            }
        }
        self.carets.merge();
    }

    pub fn edit(&mut self, range: &Range<Absolute>, text: &str) {
        let insert_index = self.rope.byte_to_char(range.start.into());
        let end_index = self.rope.byte_to_char(range.end.into());
        let cr = insert_index..end_index;
        self.rope.remove(cr);
        self.rope.insert(insert_index, text);

        for i in 0..self.carets.len() {
            let b = self.clone();
            self.carets[i].update_after_delete(range.start, range.end - range.start, &b); // TODO verify this
            let b = self.clone();
            self.carets[i].update_after_insert(range.start, text.len().into(), &b);
        }
        self.carets.merge();
        self.uuid = Uuid::new_v4();
    }

    pub fn has_many_carets(&self) -> bool {
        self.carets.len() > 1
    }

    pub fn cancel_mutli_carets(&mut self) {
        self.carets.retain(|c| !c.is_clone);
        self.carets.merge();
    }

    pub(super) fn slice<R>(&self, r: R) -> RopeSlice
    where
        R: RangeBounds<Absolute>,
    {
        let start = start_bound_to_num(r.start_bound()).unwrap_or_else(|| Absolute::from(0));
        let end = end_bound_to_num(r.end_bound()).unwrap_or_else(|| self.len());

        self.rope
            .slice(self.rope.byte_to_char(start.index)..self.rope.byte_to_char(end.index))
    }

    fn search_next_in_range(&self, s: &str, r: Range<Absolute>) -> Option<Absolute> {
        let mut index = r.start;
        let slice = self.slice(r);
        slice.lines().find_map(|l| match l.to_string().find(s) {
            Some(i) => Some(index + i),
            None => {
                index += l.len_bytes();
                None
            }
        })
    }

    pub fn search_next(&mut self, s: &str) {
        let start_index = self.main_caret().index;
        dbg!(start_index);
        let i = self
            .search_next_in_range(s, start_index..self.len())
            .or_else(|| self.search_next_in_range(s, 0.into()..start_index));
        if let Some(i) = i {
            self.cancel_mutli_carets();
            self.move_main_caret_to(i, false);
            self.move_main_caret_to(i + s.len(), true);
        }
    }

    pub fn main_caret(&self) -> &Caret {
        self.carets.iter().find(|c| !c.is_clone).expect("No main cursor found!")
    }

    pub fn main_caret_mut(&mut self) -> &mut Caret {
        self.carets
            .iter_mut()
            .find(|c| c.is_clone)
            .expect("No main cursor found!")
    }

    pub fn move_main_caret_to<P>(&mut self, pos: P, expand_selection: bool)
    where
        P: Position,
    {
        let p = self.position_to_absolute(pos);
        self.cancel_mutli_carets();
        let b = self.clone();
        self.carets[0].set_index(p, !expand_selection, true, &b)
    }

    pub fn selected_text(&self, line_feed: LineFeed) -> String {
        let mut s = String::new();
        let multi = self.carets.len() > 1;
        for c in self.carets.iter() {
            for chuck in self
                .rope
                .slice(self.rope.byte_to_char(c.start().index)..self.rope.byte_to_char(c.end().index))
                .chunks()
            {
                s.push_str(chuck)
            }
            if multi {
                s.push_str(&line_feed.to_str())
            }
        }
        s
    }

    pub fn cancel_selection(&mut self) {
        for c in &mut self.carets.iter_mut() {
            c.cancel_selection();
        }
    }

    pub fn select_all(&mut self) {
        self.cancel_mutli_carets();
        self.cancel_selection();
        self.move_main_caret_to(Absolute::from(0), false);
        self.move_main_caret_to(self.len(), true);
    }

    pub fn select_line(&mut self, line: Line, expand_selection: bool) {
        self.cancel_mutli_carets();

        if !expand_selection {
            self.cancel_selection();
            self.move_main_caret_to(line.start(&self), false);
            self.move_main_caret_to(line.end(&self), true);
        } else if self.main_caret().start() == self.main_caret().index {
            self.move_main_caret_to(line.start(&self), true);
        } else {
            self.move_main_caret_to(line.end(&self), true);
        }
    }

    pub fn caret_display_info(&self) -> String {
        if !self.has_many_carets() {
            format!(
                "Ln {}, Col {}",
                self.carets[0].line().index + 1,
                self.carets[0].col().index + 1
            )
        } else {
            format!("{} selections", self.carets.len())
        }
    }
}

impl ToString for Buffer {
    fn to_string(&self) -> String {
        self.rope.to_string()
    }
}

#[inline(always)]
pub(super) fn start_bound_to_num(b: Bound<&Absolute>) -> Option<Absolute> {
    match b {
        Bound::Included(n) => Some(*n),
        Bound::Excluded(n) => Some(*n + 1),
        Bound::Unbounded => None,
    }
}

#[inline(always)]
pub(super) fn end_bound_to_num(b: Bound<&Absolute>) -> Option<Absolute> {
    match b {
        Bound::Included(n) => Some(*n + 1),
        Bound::Excluded(n) => Some(*n),
        Bound::Unbounded => None,
    }
}
