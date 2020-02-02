use crate::rope_utils::*;
use ropey::Rope;
use std::ops::{AddAssign, Range, SubAssign};

#[derive(Debug, Clone)]
pub struct Carret {
    pub index: usize,
    vcol: usize,
    col_index: usize,
    pub selection: Option<usize>,
    pub is_clone: bool,
}

impl Carret {
    pub fn new() -> Self {
        Self {
            index: 0,
            vcol: 0,
            col_index: 0,
            selection: Default::default(),
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
                selection: Some(cend.range().end),
                is_clone: cstart.is_clone && cend.is_clone,
            }
        } else {
            let (cstart, cend) = if c1.end() < c2.end() { (c1, c2) } else { (c2, c1) };
            Self {
                index: cend.end(),
                vcol: cend.vcol,
                col_index: cend.col_index,
                selection: Some(cstart.start()),
                is_clone: cstart.is_clone && cend.is_clone,
            }
        }
    }

    pub fn collide_with(&self, c1: &Carret) -> bool {
        self.range().contains(&c1.range().start) || (self.selection.is_none() && self.index == c1.index)
    }

    pub fn range(&self) -> Range<usize> {
        if let Some(selection) = self.selection {
            if selection < self.index {
                return selection..self.index;
            } else {
                return self.index..selection;
            }
        } else {
            return self.index..self.index;
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

    pub fn index_line(&self, rope: &Rope) -> usize {
        rope.byte_to_line(self.index)
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
            if expand_selection {
                if self.selection.is_none() {
                    self.selection = Some(self.index);
                }
            } else {
                self.selection = None;
            };
            self.index = self.index_from_top_neighbor(rope);
            self.recalculate_col_index(rope);
        }
    }

    pub fn move_down(&mut self, expand_selection: bool, rope: &Rope) {
        let line = self.index_line(rope);
        if line < rope.len_lines() - 1 {
            if expand_selection {
                if self.selection.is_none() {
                    self.selection = Some(self.index);
                }
            } else {
                self.selection = None;
            };

            self.index = self.index_from_bottom_neighbor(rope);
            self.recalculate_col_index(rope);
        }
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
        if let Some(ref mut s) = self.selection {
            if *s > index {
                *s += delta;
            }
        }
    }
    pub fn update_after_delete(&mut self, index: usize, delta: usize, rope: &Rope) {
        if self.index > index {
            self.set_index(self.index - delta, rope);
        }

        if let Some(ref mut s) = self.selection {
            if *s > index {
                *s -= delta;
            }
        }
    }

    fn add(&mut self, delta: usize, rope: &Rope) {
        self.set_index(self.index + delta, rope);
        if let Some(ref mut s) = self.selection {
            *s += delta;
        }
    }
    fn sub(&mut self, delta: usize, rope: &Rope) {
        self.set_index(self.index - delta, rope);
        if let Some(ref mut s) = self.selection {
            *s -= delta;
        }
    }
}
