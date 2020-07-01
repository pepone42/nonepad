use super::{
    caret::{Caret, Carets},
    file::{Indentation, LineFeed},
    position::{Absolute, Column, Line, Point, Position, Relative},
    SelectionLineRange, rope_utils,
};
use druid::Data;
use ropey::{Rope, RopeSlice};
use std::ops::Range;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Buffer {
    rope: Rope,
    carets: Carets,
    tabsize: usize,
    uuid: Uuid,
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
        }
    }

    pub fn from_rope(rope: Rope, tabsize: usize) -> Self {
        Self {
            rope: rope.clone(),
            carets: Carets::new(),
            uuid: Uuid::new_v4(),
            tabsize,
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
        line.displayable_string(
            &self.rope,
            self.tabsize,
            out,
            rel_to_byte,
            byte_to_rel,
        );
    }

    pub fn carets_on_line<'a>(&'a self, line: Line) -> impl Iterator<Item = &'a Caret> {
        self.carets.iter().filter(move |c| c.line() == line)
    }

    pub fn selection_on_line<'a>(&'a self, line_idx: usize, ranges: &mut Vec<SelectionLineRange>) {
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

    pub fn point<C, L>(&self, col: C, line: L) -> Point
    where
        C: Into<Column>,
        L: Into<Line>,
    {
        Point::new(col.into(), line.into(), &self.rope, self.tabsize)
    }

    pub fn backward(&mut self, expand_selection: bool, word_boundary: bool) {
        for s in &mut self.carets.iter_mut() {
            s.move_backward(
                expand_selection,
                word_boundary,
                &self.rope,
                self.tabsize,
            );
        }

        self.carets.merge();
    }

    pub fn forward(&mut self, expand_selection: bool, word_boundary: bool) {
        for s in &mut self.carets.iter_mut() {
            s.move_forward(
                expand_selection,
                word_boundary,
                &self.rope,
                self.tabsize,
            );
        }

        self.carets.merge();
    }

    pub fn up(&mut self, expand_selection: bool) {
        for s in &mut self.carets.iter_mut() {
            s.move_up(expand_selection, &self.rope, self.tabsize);
        }

        self.carets.merge();
    }
    pub fn down(&mut self, expand_selection: bool) {
        for s in &mut self.carets.iter_mut() {
            s.move_down(expand_selection, &self.rope, self.tabsize);
        }

        self.carets.merge();
    }
    pub fn duplicate_down(&mut self) {
        self.carets.sort_unstable();

        if let Some(c) = self.carets.last().and_then(|c| c.duplicate_down(&self.rope, self.tabsize)) {
            self.carets.push(c);
        }
        self.carets.merge();
    }

    pub fn duplicate_up(&mut self) {
        self.carets.sort_unstable();

        if let Some(c) = self.carets.first().and_then(|c| c.duplicate_up(&self.rope, self.tabsize)) {
            self.carets.push(c);
        }
        self.carets.merge();
    }

    pub fn have_selection(&self) -> bool {
        self.carets.iter().any(|c| !c.selection_is_empty())
    }

    pub fn end(&mut self, expand_selection: bool) {
        for s in &mut self.carets.iter_mut() {
            s.move_end(expand_selection, &self.rope, self.tabsize);
        }

        self.carets.merge();
    }

    pub fn home(&mut self, expand_selection: bool) {
        for s in &mut self.carets.iter_mut() {
            s.move_home(expand_selection, &self.rope, self.tabsize);
        }
        self.carets.merge();
    }

    pub fn insert(&mut self, text: &str) {
        for i in 0..self.carets.len() {
            let r = self.carets[i].range();
            self.edit(&r, text, self.tabsize);
            self.carets[i].set_index(r.start + text.len(), true, true, &self.rope, self.tabsize);
        }
        self.carets.merge();
    }

    pub fn backspace(&mut self) -> bool {
        let mut did_nothing = true;
        for i in 0..self.carets.len() {
            if !self.carets[i].selection_is_empty() {
                // delete all the selection
                let r = self.carets[i].range();
                self.edit(&r, "", self.tabsize);
                self.carets[i].set_index(r.start, true, true, &self.rope, self.tabsize);

                did_nothing = false;
            } else if self.carets[i].index > 0.into() {
                // delete the preceding grapheme
                let r = rope_utils::prev_grapheme_boundary(&self.rope.slice(..), self.carets[i].index).into()
                    ..self.carets[i].index;
                self.edit(&r, "", self.tabsize);
                self.carets[i].set_index(r.start, true, true, &self.rope, self.tabsize);

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
                self.edit(&r, "", self.tabsize);
                self.carets[i].set_index(r.start, true, true, &self.rope, self.tabsize);

                did_nothing = false;
            } else if self.carets[i].index < self.rope.len_bytes().into() {
                let r = self.carets[i].index
                    ..rope_utils::next_grapheme_boundary(&self.rope.slice(..), self.carets[i].index).into();
                self.edit(&r, "", self.tabsize);
                self.carets[i].set_index(r.start, true, true, &self.rope, self.tabsize);

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
            if let Some(line_range) = self.carets[i].selected_lines_range(&self.rope) {
                // TODO: Find a better way to iterate over line of a selection
                for line_idx in line_range.start().index..line_range.end().index + 1 {
                    let line_start: Absolute = self.rope.line_to_byte(line_idx).into();
                    let r = line_start..line_start;
                    let text = match indentation {
                        Indentation::Space(n) => " ".repeat(n).to_owned(),
                        Indentation::Tab(_) => "\t".to_owned(),
                    };
                    self.edit(&r, &text, self.tabsize);
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
                self.edit(&r, &text, self.tabsize);
                self.carets[i].set_index(r.start + Relative::from(text.len()), true, true, &self.rope, self.tabsize);
            }
        }
        self.carets.merge();
    }

    pub fn edit(&mut self, range: &Range<Absolute>, text: &str, tabsize: usize) {
        let insert_index = self.rope.byte_to_char(range.start.into());
        let end_index = self.rope.byte_to_char(range.end.into());
        let cr = insert_index..end_index;
        self.rope.remove(cr);
        self.rope.insert(insert_index, text);

        for i in 0..self.carets.len() {
            self.carets[i].update_after_delete(range.start, (range.end - range.start).into(), &self.rope, tabsize); // TODO verify this
            self.carets[i].update_after_insert(range.start, text.len().into(), &self.rope, tabsize);
        }
        self.carets.merge();
        self.uuid = Uuid::new_v4();
    }

    pub fn position_to_absolute<P>(&self, pos: P) -> Absolute
    where
        P: Position,
    {
        pos.absolute(&self.rope, self.tabsize)
    }
    pub fn position_to_point<P>(&self, pos: P) -> Point
    where
        P: Position,
    {
        pos.point(&self.rope, self.tabsize)
    }

    pub fn len(&self) -> Absolute {
        self.rope.len_bytes().into()
    }

    pub fn len_lines(&self) -> usize {
        self.rope.len_lines()
    }

    pub fn has_many_carets(&self) -> bool {
        self.carets.len() > 1
    }

    pub fn cancel_mutli_carets(&mut self) {
        self.carets.retain(|c| !c.is_clone);
        self.carets.merge();
    }

    fn slice(&self, r: Range<Absolute>) -> RopeSlice {
        self.rope
            .slice(self.rope.byte_to_char(r.start.index)..self.rope.byte_to_char(r.end.index))
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
            .or(self.search_next_in_range(s, 0.into()..start_index));
        if let Some(i) = i {
            self.cancel_mutli_carets();
            self.move_main_caret_to(i, false);
            self.move_main_caret_to(i + s.len(), true);
        }
    }

    pub fn main_caret(&self) -> &Caret {
        self.carets
            .iter()
            .filter(|c| !c.is_clone)
            .nth(0)
            .expect("No main cursor found!")
    }

    pub fn main_caret_mut(&mut self) -> &mut Caret {
        self.carets
            .iter_mut()
            .filter(|c| c.is_clone == false)
            .nth(0)
            .expect("No main cursor found!")
    }

    pub fn move_main_caret_to<P>(&mut self, pos: P, expand_selection: bool)
    where
        P: Position,
    {
        let p = self.position_to_absolute(pos);
        self.cancel_mutli_carets();
        self.carets[0].set_index(
            p,
            !expand_selection,
            true,
            &self.rope,
            self.tabsize,
        )
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
