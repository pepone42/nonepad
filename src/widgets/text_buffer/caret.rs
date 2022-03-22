use super::position::{Absolute, Column, Line, Point, Position, Relative};
use super::{buffer::Buffer, rope_utils::*};
use druid::Data;

use std::ops::Deref;
use std::ops::DerefMut;
use std::ops::{Range, RangeInclusive};

#[derive(Debug)]
pub struct Carets {
    intern: Vec<Caret>,
}

impl Carets {
    pub fn new() -> Self {
        Self {
            intern: vec![Caret::new()],
        }
    }

    pub fn merge(&mut self) {
        if self.intern.len() > 1 {
            self.intern
                .sort_unstable_by(|a, b| a.range().start.cmp(&b.range().start))
        }
        let mut redo = true;
        'outer: while redo {
            for i in 0..self.intern.len() - 1 {
                if self.intern[i].collide_with(&self.intern[i + 1]) {
                    self.intern[i] = Caret::merge(&self.intern[i], &self.intern[i + 1]);
                    self.intern.remove(i + 1);
                    redo = true;
                    continue 'outer;
                }
            }
            redo = false;
        }
    }
}

impl Default for Carets {
    fn default() -> Self {
        Self::new()
    }
}

impl Data for Carets {
    fn same(&self, other: &Self) -> bool {
        if self.intern.len() != other.intern.len() {
            return false;
        }
        for i in 0..self.intern.len() {
            if !self.intern[i].same(&other.intern[i]) {
                return false;
            }
        }
        true
    }
}

impl Clone for Carets {
    fn clone(&self) -> Self {
        let mut intern: Vec<Caret> = self.intern.iter().filter(|c| c.is_clone).cloned().collect();
        let mut first = self.intern.iter().find(|c| !c.is_clone).unwrap().clone();
        first.is_clone = false;
        intern.push(first);
        intern.sort_unstable();
        Self { intern }
    }
}

impl Deref for Carets {
    type Target = Vec<Caret>;
    fn deref(&self) -> &Self::Target {
        &self.intern
    }
}
impl DerefMut for Carets {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.intern
    }
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Caret {
    pub index: Absolute,
    selection: Absolute,
    point: Point,
    sticky_col: Column,
    pub is_clone: bool,
    pub(super) generation: usize,
}

impl Data for Caret {
    fn same(&self, other: &Self) -> bool {
        self.index == other.index &&
        self.selection == other.selection
    }
}

impl Clone for Caret {
    fn clone(&self) -> Self {
        Self {
            index: self.index,
            selection: self.selection,
            point: self.point,
            sticky_col: self.sticky_col,
            is_clone: true,
            generation: self.generation + 1,
        }
    }
}

impl PartialOrd for Caret {
    fn partial_cmp(&self, other: &Caret) -> Option<core::cmp::Ordering> {
        Some(self.index.cmp(&other.index))
    }
}

impl Ord for Caret {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.index.cmp(&other.index)
    }
}

impl Caret {
    pub fn new() -> Self {
        Self {
            index: Default::default(),
            selection: Default::default(),
            point: Default::default(),
            sticky_col: Default::default(),
            is_clone: false,
            generation: 0,
        }
    }
    fn from_point(p: Point, buffer: &Buffer) -> Self {
        Self {
            index: p.absolute(buffer),
            selection: p.absolute(buffer),
            point: p,
            sticky_col: p.col,
            is_clone: false,
            generation: 0,
        }
    }

    pub fn merge(c1: &Caret, c2: &Caret) -> Self {
        if c1.index < c1.selection {
            let (cstart, cend) = if c1.start() < c2.start() { (c1, c2) } else { (c2, c1) };
            Self {
                index: cstart.index,
                point: cstart.point,
                sticky_col: cstart.sticky_col,
                selection: cend.end(),
                is_clone: cstart.is_clone && cend.is_clone,
                generation: cstart.generation.max(cend.generation),
            }
        } else {
            let (cstart, cend) = if c1.end() < c2.end() { (c1, c2) } else { (c2, c1) };
            Self {
                index: cend.end(),
                point: cend.point,
                sticky_col: cend.sticky_col,
                selection: cstart.start(),
                is_clone: cstart.is_clone && cend.is_clone,
                generation: cstart.generation.max(cend.generation),
            }
        }
    }

    pub fn cancel_selection(&mut self) {
        self.selection = self.index;
    }

    pub fn selection_is_empty(&self) -> bool {
        self.selection == self.index
    }

    pub fn collide_with(&self, c1: &Caret) -> bool {
        self.range().contains(&c1.range().start) || (self.selection_is_empty() && self.index == c1.index)
    }

    pub fn range(&self) -> Range<Absolute> {
        if self.selection < self.index {
            self.selection..self.index
        } else {
            self.index..self.selection
        }
    }

    pub fn start(&self) -> Absolute {
        self.range().start
    }

    pub fn end(&self) -> Absolute {
        self.range().end
    }

    pub fn start_line(&self, buffer: &Buffer) -> Line {
        self.start().line(buffer)
    }

    pub fn end_line(&self, buffer: &Buffer) -> Line {
        self.end().line(buffer)
    }

    pub fn selected_lines_range(&self, buffer: &Buffer) -> Option<RangeInclusive<Line>> {
        if self.selection_is_empty() {
            None
        } else {
            Some(self.start_line(buffer)..=self.end_line(buffer))
        }
    }

    pub fn line(&self) -> Line {
        self.point.line
    }

    pub fn relative(&self) -> Relative {
        self.point.relative
    }

    pub fn col(&self) -> Column {
        self.point.col
    }

    pub fn set_index(&mut self, index: Absolute, reset_selection: bool, reset_sticky_col: bool, buffer: &Buffer) {
        self.index = index;
        if reset_selection {
            self.selection = index;
        }
        self.point = self.index.point(buffer);
        if reset_sticky_col {
            self.sticky_col = self.point.col;
        }
    }

    pub fn move_up(&mut self, expand_selection: bool, buffer: &Buffer) {
        let pos = Point::new(self.sticky_col, self.line(), buffer).up(buffer);
        self.set_index(pos.absolute(buffer), !expand_selection, false, buffer);
    }

    pub fn move_down(&mut self, expand_selection: bool, buffer: &Buffer) {
        let pos = Point::new(self.sticky_col, self.line(), buffer).down(buffer);
        self.set_index(pos.absolute(buffer), !expand_selection, false, buffer);
    }

    pub fn move_backward(&mut self, expand_selection: bool, word_boundary: bool, buffer: &Buffer) {
        let index = if word_boundary {
            prev_word_boundary(&buffer.slice(..), self.index)
        } else {
            prev_grapheme_boundary(&buffer.slice(..), self.index)
        };
        self.set_index(Absolute::from(index), !expand_selection, true, buffer);
    }

    pub fn move_forward(&mut self, expand_selection: bool, word_boundary: bool, buffer: &Buffer) {
        let index = if word_boundary {
            next_word_boundary(&buffer.slice(..), self.index)
        } else {
            next_grapheme_boundary(&buffer.slice(..), self.index)
        };
        self.set_index(Absolute::from(index), !expand_selection, true, buffer);
    }

    pub fn duplicate_down(&self, buffer: &Buffer) -> Option<Self> {
        if self.line().next(buffer).is_some() {
            let mut c = Caret::from_point(self.point.down(buffer), buffer);
            c.is_clone = true;
            c.generation = self.generation + 1;
            Some(c)
        } else {
            None
        }
    }

    pub fn duplicate_up(&self, buffer: &Buffer) -> Option<Self> {
        if self.line().prev().is_some() {
            let mut c = Caret::from_point(self.point.up(buffer), buffer);
            c.is_clone = true;
            c.generation = self.generation + 1;
            Some(c)
        } else {
            None
        }
    }

    pub fn duplicate_to(&self, start: Absolute, end: Absolute, buffer: &Buffer) -> Self {
        let mut c = Caret::new();
        c.set_index(start, true, true, buffer);
        c.set_index(end, false, true, buffer);
        c.is_clone = true;
        c.generation = self.generation + 1;
        c
    }

    pub fn move_end(&mut self, expand_selection: bool, buffer: &Buffer) {
        let index = self.line().end(buffer);
        self.set_index(index, !expand_selection, true, buffer);
    }

    pub fn move_home(&mut self, expand_selection: bool, buffer: &Buffer) {
        let index = self.line().start(buffer);
        let index2 = self.line().absolute_indentation(buffer);
        let index = if self.index>index2 || self.index == index {
            index2
        } else {
            index
        };
        self.set_index(index, !expand_selection, true, buffer);
    }

    pub fn update_after_insert(&mut self, index: Absolute, delta: Relative, buffer: &Buffer) {
        if self.index > index {
            let col = self.col();
            self.set_index(self.index + delta, false, false, buffer);
            // Update virtal column position only if the real column position changed
            if col != self.col() {
                self.sticky_col = col;
            }
        }
        if self.selection > index {
            self.selection += delta;
        }
    }
    pub fn update_after_delete(&mut self, index: Absolute, delta: Relative, buffer: &Buffer) {
        if self.index > index {
            let col = self.col();
            self.set_index(self.index - delta, false, false, buffer);
            // Update virtal column position only if the real column position changed
            if col != self.col() {
                self.sticky_col = col;
            }
        }

        if self.selection > index {
            self.selection -= delta;
        }
    }
}
