use crate::position::{Absolute, Column, Line, Point, Position, Relative};
use crate::rope_utils::*;
use druid::Data;
use ropey::Rope;

use std::ops::Deref;
use std::ops::DerefMut;
use std::ops::{Range, RangeInclusive};

#[derive(Debug, PartialEq, Eq)]
pub struct Carrets {
    intern: Vec<Carret>,
}

impl Carrets {
    pub fn new() -> Self {
        let mut intern = Vec::new();
        intern.push(Carret::new());
        Self { intern }
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
                    self.intern[i] = Carret::merge(&self.intern[i], &self.intern[i + 1]);
                    self.intern.remove(i + 1);
                    redo = true;
                    continue 'outer;
                }
            }
            redo = false;
        }
    }
}

impl Clone for Carrets {
    fn clone(&self) -> Self {
        let mut intern: Vec<Carret> = self.intern.iter().filter(|c| c.is_clone).map(|c| c.clone()).collect();
        let mut first = self.intern.iter().filter(|c| !c.is_clone).nth(0).unwrap().clone();
        first.is_clone = false;
        intern.push(first);
        intern.sort_unstable();
        Self { intern }
    }
}

impl Deref for Carrets {
    type Target = Vec<Carret>;
    fn deref(&self) -> &Self::Target {
        &self.intern
    }
}
impl DerefMut for Carrets {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.intern
    }
}

#[derive(Debug, PartialEq, Eq, Default, Data)]
pub struct Carret {
    // pub index: AbsoluteIndex,
    // col: Column,
    // pub line: usize,
    // vcol: Column,
    // pub rel_index: RelativeIndex,
    // selection: AbsoluteIndex,
    pub index: Absolute,
    selection: Absolute,
    point: Point,
    sticky_col: Column,
    pub is_clone: bool,
}

impl Clone for Carret {
    fn clone(&self) -> Self {
        Self {
            index: self.index,
            selection: self.selection,
            point: self.point,
            sticky_col: self.sticky_col,
            is_clone: true,
        }
    }
}

impl PartialOrd for Carret {
    fn partial_cmp(&self, other: &Carret) -> Option<core::cmp::Ordering> {
        Some(self.index.cmp(&other.index))
    }
}

impl Ord for Carret {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.index.cmp(&other.index)
    }
}

impl Carret {
    pub fn new() -> Self {
        Self {
            index: Default::default(),
            selection: Default::default(),
            point: Default::default(),
            sticky_col: Default::default(),
            is_clone: false,
        }
    }
    fn from_point(p: Point, rope: &Rope, tabsize: usize) -> Self {
        Self {
            index: p.absolute(rope, tabsize),
            selection: p.absolute(rope, tabsize),
            point: p,
            sticky_col: p.col,
            is_clone: false,
        }
    }

    pub fn merge(c1: &Carret, c2: &Carret) -> Self {
        if c1.index < c1.selection {
            let (cstart, cend) = if c1.start() < c2.start() { (c1, c2) } else { (c2, c1) };
            Self {
                index: cstart.index,
                point: cstart.point,
                sticky_col: cstart.sticky_col,
                selection: cend.end(),
                is_clone: cstart.is_clone && cend.is_clone,
            }
        } else {
            let (cstart, cend) = if c1.end() < c2.end() { (c1, c2) } else { (c2, c1) };
            Self {
                index: cend.end(),
                point: cend.point,
                sticky_col: cend.sticky_col,
                selection: cstart.start(),
                is_clone: cstart.is_clone && cend.is_clone,
            }
        }
    }

    pub fn cancel_selection(&mut self) {
        self.selection = self.index;
    }

    pub fn selection_is_empty(&self) -> bool {
        self.selection == self.index
    }

    pub fn collide_with(&self, c1: &Carret) -> bool {
        self.range().contains(&c1.range().start) || (self.selection_is_empty() && self.index == c1.index)
    }

    pub fn range(&self) -> Range<Absolute> {
        if self.selection < self.index {
            return self.selection..self.index;
        } else {
            return self.index..self.selection;
        }
    }

    pub fn start(&self) -> Absolute {
        self.range().start
    }

    pub fn end(&self) -> Absolute {
        self.range().end
    }

    pub fn start_line(&self, rope: &Rope) -> Line {
        self.start().line(rope)
    }

    pub fn end_line(&self, rope: &Rope) -> Line {
        self.end().line(rope)
    }

    pub fn selected_lines_range(&self, rope: &Rope) -> Option<RangeInclusive<Line>> {
        if self.selection_is_empty() {
            None
        } else {
            Some(self.start_line(rope)..=self.end_line(rope))
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

    pub fn set_index(
        &mut self,
        index: Absolute,
        reset_selection: bool,
        reset_sticky_col: bool,
        rope: &Rope,
        tabsize: usize,
    ) {
        self.index = index;
        if reset_selection {
            self.selection = index;
        }
        self.point = self.index.point(rope, tabsize);
        if reset_sticky_col {
            self.sticky_col = self.point.col;
        }
    }

    pub fn move_up(&mut self, expand_selection: bool, rope: &Rope, tabsize: usize) {
        let pos = Point::new(self.sticky_col, self.line(), rope, tabsize).up(rope, tabsize);
        self.set_index(pos.absolute(rope, tabsize), !expand_selection, false, rope, tabsize);
    }

    pub fn move_down(&mut self, expand_selection: bool, rope: &Rope, tabsize: usize) {
        let pos = Point::new(self.sticky_col, self.line(), rope, tabsize).down(rope, tabsize);
        self.set_index(pos.absolute(rope, tabsize), !expand_selection, false, rope, tabsize);
    }

    pub fn move_backward(&mut self, expand_selection: bool, word_boundary: bool, rope: &Rope, tabsize: usize) {
        let index = if word_boundary {
            prev_word_boundary(&rope.slice(..), self.index)
        } else {
            prev_grapheme_boundary(&rope.slice(..), self.index)
        };
        self.set_index(Absolute::from(index), !expand_selection, true, rope, tabsize);
    }

    pub fn move_forward(&mut self, expand_selection: bool, word_boundary: bool, rope: &Rope, tabsize: usize) {
        let index = if word_boundary {
            next_word_boundary(&rope.slice(..), self.index)
        } else {
            next_grapheme_boundary(&rope.slice(..), self.index)
        };
        self.set_index(Absolute::from(index), !expand_selection, true, rope, tabsize);
    }

    pub fn duplicate_down(&self, rope: &Rope, tabsize: usize) -> Option<Self> {
        if self.line().next(rope).is_some() {
            let mut c = Carret::from_point(self.point.down(rope, tabsize), rope, tabsize);
            c.is_clone = true;
            Some(c)
        } else {
            None
        }
    }

    pub fn duplicate_up(&self, rope: &Rope, tabsize: usize) -> Option<Self> {
        if self.line().prev().is_some() {
            let mut c = Carret::from_point(self.point.up(rope, tabsize), rope, tabsize);
            c.is_clone = true;
            Some(c)
        } else {
            None
        }
    }

    pub fn move_end(&mut self, expand_selection: bool, rope: &Rope, tabsize: usize) {
        let index = self.line().end(rope);
        self.set_index(index, !expand_selection, true, rope, tabsize);
    }

    pub fn move_home(&mut self, expand_selection: bool, rope: &Rope, tabsize: usize) {
        let index = self.line().start(rope);
        self.set_index(index, !expand_selection, true, rope, tabsize);
    }

    pub fn update_after_insert(&mut self, index: Absolute, delta: Relative, rope: &Rope, tabsize: usize) {
        if self.index > index {
            let col = self.col();
            self.set_index(self.index + delta, false, false, rope, tabsize);
            // Update virtal column position only if the real column position changed
            if col != self.col() {
                self.sticky_col = col;
            }
        }
        if self.selection > index {
            self.selection += delta;
        }
    }
    pub fn update_after_delete(&mut self, index: Absolute, delta: Relative, rope: &Rope, tabsize: usize) {
        if self.index > index {
            let col = self.col();
            self.set_index(self.index - delta, false, false, rope, tabsize);
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
