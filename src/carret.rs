use crate::rope_utils::*;
use ropey::Rope;
use std::ops::{AddAssign, Range, SubAssign};
use crate::rope_utils::{AbsoluteIndex,RelativeIndex,Column};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Carret {
    index: AbsoluteIndex,
    vcol: Column,
    col_index: RelativeIndex,
    selection: AbsoluteIndex,
    pub is_clone: bool,
}

impl PartialOrd for Carret {
    fn partial_cmp(&self, other: &Carret) -> Option<core::cmp::Ordering> { Some(self.index.cmp(&other.index)) }
}

impl Ord for Carret {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering { self.index.cmp(&other.index) }
    
}

impl Carret {
    pub fn new() -> Self {
        Self {
            index: 0,
            vcol: 0,
            col_index: 0,
            selection: 0,
            is_clone: false,
        }
    }

    pub fn merge(c1: &Carret, c2: &Carret) -> Self {
        if c1.index == c1.start() {
            let (cstart, cend) = if c1.start() < c2.start() { (c1, c2) } else { (c2, c1) };
            Self {
                index: cstart.index,
                vcol: cstart.vcol,
                col_index: cstart.col_index,
                selection: cend.end(),
                is_clone: cstart.is_clone && cend.is_clone,
            }
        } else {
            let (cstart, cend) = if c1.end() < c2.end() { (c1, c2) } else { (c2, c1) };
            Self {
                index: cend.end(),
                vcol: cend.vcol,
                col_index: cend.col_index,
                selection: cstart.start(),
                is_clone: cstart.is_clone && cend.is_clone,
            }
        }
    }

    pub fn cancel_selection(&mut self) {
        self.selection = self.index;
    }

    pub fn selection_is_empty(&self) -> bool {
        self.selection != self.index
    }

    pub fn collide_with(&self, c1: &Carret) -> bool {
        self.range().contains(&c1.range().start) || (self.selection_is_empty() && self.index == c1.index)
    }

    pub fn range(&self) -> Range<usize> {
        if self.selection < self.index {
            return self.selection..self.index;
        } else {
            return self.index..self.selection;
        }
    }

    pub fn start(&self) -> usize {
        self.range().start
    }

    pub fn end(&self) -> usize {
        self.range().end
    }

    pub fn start_line(&self, rope: &Rope) -> usize {
        rope.byte_to_line(self.start())
    }

    pub fn end_line(&self, rope: &Rope) -> usize {
        rope.byte_to_line(self.end())
    }

    pub fn line(&self, rope: &Rope) -> usize {
        //rope.byte_to_line(self.index)
        crate::rope_utils::Line::for_index(self.index);
    }

    pub fn selected_lines(&self, rope: &Rope) -> Option<Range<usize>> {
        match (self.start_line(rope), self.end_line(rope)) {
            (s, e) if s == e => None,
            (s, e) => Some(s..e),
        }
    }

    pub fn index_column(&self) -> usize {
        self.col_index
    }

    fn recalculate_col_index(&mut self, rope: &Rope) {
        self.col_index = self.index - rope.line_to_byte(self.index_line(rope));
    }

    fn recalculate_vcol(&mut self, rope: &Rope) {
        let (vcol, line) = index_to_point(&rope.slice(..), self.index);
        self.vcol = vcol;
    }

    pub fn set_index(&mut self, index: usize, rope: &Rope) {
        self.index = index;
        self.recalculate_vcol(rope);
        self.recalculate_col_index(rope);
    }

    fn index_from_top_neighbor(&self, rope: &Rope) -> usize {
        point_to_index(&rope.slice(..), self.vcol, self.index_line(rope) - 1).0
    }
    fn index_from_bottom_neighbor(&self, rope: &Rope) -> usize {
        point_to_index(&rope.slice(..), self.vcol, self.index_line(rope) + 1).0
    }

    pub fn move_up(&mut self, expand_selection: bool, rope: &Rope) {
        //let line = rope.byte_to_line(s.index);
        if self.index_line(rope) > 0 {
            // if expand_selection {
            //     if self.selection_is_empty() {
            //         self.selection = self.index;
            //     }
            // } else {
            //     self.selection = self.index;
            // };
            if !expand_selection {
                self.selection = self.index;
            }
            self.index = self.index_from_top_neighbor(rope);
            self.recalculate_col_index(rope);
        }
    }

    pub fn move_down(&mut self, expand_selection: bool, rope: &Rope) {
        let line = self.index_line(rope);
        if line < rope.len_lines() - 1 {
            if !expand_selection {
                self.selection = self.index;
            }

            self.index = self.index_from_bottom_neighbor(rope);
            self.recalculate_col_index(rope);
        }
    }

    pub fn move_backward(&mut self, expand_selection: bool, rope: &Rope) {
        let index = prev_grapheme_boundary(&rope.slice(..), self.index);

        if !expand_selection {
            self.selection = self.index;
        }
        self.set_index(index, &rope);
    }

    pub fn move_forward(&mut self, expand_selection: bool, rope: &Rope) {
        let index = next_grapheme_boundary(&rope.slice(..), self.index);

        if !expand_selection {
            self.selection = self.index;
        }
        self.set_index(index, &rope);
    }

    pub fn duplicate_down(&self, rope: &Rope) -> Option<Self> {
        if self.index_line(rope) < rope.len_lines() - 1 {
            let mut c = self.clone();
            c.is_clone = true; // TODO: impl Clone?

            let i = self.index_from_bottom_neighbor(rope);
            c.add(i - c.index, &rope);

            Some(c)
        } else {
            None
        }
    }

    pub fn duplicate_up(&self, rope: &Rope) -> Option<Self> {
        if self.index_line(rope) > 0 {
            let mut c = self.clone();
            c.is_clone = true; // TODO: impl Clone?

            let i = self.index_from_top_neighbor(rope);
            c.sub(c.index - i, &rope);

            Some(c)
        } else {
            None
        }
    }

    pub fn update_after_insert(&mut self, index: usize, delta: usize, rope: &Rope) {
        if self.index > index {
            self.set_index(self.index + delta, rope);
        }
        if self.selection > index {
            self.selection += delta;
        }
    }
    pub fn update_after_delete(&mut self, index: usize, delta: usize, rope: &Rope) {
        if self.index > index {
            self.set_index(self.index - delta, rope);
        }

        if self.selection > index {
            self.selection -= delta;
        }
    }

    fn add(&mut self, delta: usize, rope: &Rope) {
        self.set_index(self.index + delta, rope);
        self.selection += delta;
    }
    fn sub(&mut self, delta: usize, rope: &Rope) {
        self.set_index(self.index - delta, rope);
        self.selection -= delta;
    }
}
