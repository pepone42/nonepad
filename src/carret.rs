use crate::rope_utils::*;
use crate::rope_utils::{AbsoluteIndex, Column, RelativeIndex};
use ropey::{Rope, RopeSlice};
use std::ops::{AddAssign, Range, SubAssign, RangeInclusive};

#[derive(Debug,Clone)]
pub struct CarretMut<'rs> {
    carret: Carret,
    owner: RopeSlice<'rs>,
    tabsize: usize,
}


impl<'rs> CarretMut<'rs> {
    pub fn new(carret: Carret, owner: RopeSlice<'rs>, tabsize: usize) -> Self {
        Self { carret, owner, tabsize }
    }

    pub fn start_line(&self) -> usize {
        self.owner.byte_to_line(self.carret.start().0)
    }

    pub fn end_line(&self) -> usize {
        self.owner.byte_to_line(self.carret.end().0)
    }

    pub fn selected_lines_range(&self) -> Option<RangeInclusive<usize>> {
        if self.carret.selection_is_empty() {
            None
        } else { 
            Some(self.start_line()..=self.end_line())
        }
    }

    pub fn line(&self) -> Line {
        //rope.byte_to_line(self.index)
        Line::for_index(self.carret.index, self.owner)
    }

    pub fn relative_index(&self) -> RelativeIndex {
        self.line().absolute_to_relative_index(self.carret.index)
    }

    pub fn column_index(&self) -> Column {
        self.line().relative_index_to_column(self.relative_index(),self.tabsize)
    }

    // fn recalculate_col_index(&mut self, rope: &Rope) {
    //     self.col_index = self.index - rope.line_to_byte(self.index_line(rope));
    // }

    fn recalculate_vcol(&mut self) {
        self.carret.vcol = self.column_index();
    }

    pub fn set_index(&mut self, index: AbsoluteIndex) {
        self.carret.index = index;
        self.carret.col = self.column_index();
        let line = self.line();
        self.carret.rel_index = line.absolute_to_relative_index(self.carret.index);
        self.carret.line = line.line;
    }

    pub fn move_up(&mut self, expand_selection: bool) {
        //let line = rope.byte_to_line(s.index);
        if let Some(prev_line) = self.line().prev_line() {
            if !expand_selection {
                self.carret.selection = self.carret.index
            }
            self.set_index(prev_line.column_to_absolute_index(self.carret.vcol,self.tabsize));
        }
    }

    pub fn move_down(&mut self, expand_selection: bool) {
        //let line = rope.byte_to_line(s.index);
        if let Some(next_line) = self.line().next_line() {
            if !expand_selection {
                self.carret.selection = self.carret.index
            }
            self.set_index(next_line.column_to_absolute_index(self.carret.vcol,self.tabsize));
        }
    }

    pub fn move_backward(&mut self, expand_selection: bool) {
        let index = prev_grapheme_boundary(&self.owner, self.carret.index.0);

        if !expand_selection {
            self.carret.selection = self.carret.index
        }
        self.set_index(AbsoluteIndex(index));
    }

    pub fn move_forward(&mut self, expand_selection: bool) {
        let index = next_grapheme_boundary(&self.owner, self.carret.index.0);

        if !expand_selection {
            self.carret.selection = self.carret.index
        }
        self.set_index(AbsoluteIndex(index));
    }

    pub fn duplicate_down(&self) -> Option<Self> {
        if let Some(next_line) = self.line().next_line() {
            let mut c = self.clone();
            c.set_index(next_line.column_to_absolute_index(self.carret.vcol,self.tabsize));
            Some(c)
        } else {
            None
        }
    }

    pub fn duplicate_up(&self, rope: &Rope) -> Option<Self> {
        if let Some(prev_line) = self.line().prev_line() {
            let mut c = self.clone();
            self.set_index(prev_line.column_to_absolute_index(self.carret.vcol,self.tabsize));
            Some(c)
        } else {
            None
        }
    }

    pub fn update_after_insert(&mut self, index: AbsoluteIndex, delta: usize) {
        if self.carret.index > index {
            let col = self.carret.col;
            self.set_index(self.carret.index + delta);
            // Update virtal column position only if the real column position changed
            if col!=self.carret.col {
                self.carret.vcol = col;
            }
        }
        if self.carret.selection > index {
            self.carret.selection += delta;
        }
    }
    pub fn update_after_delete(&mut self, index: AbsoluteIndex, delta: usize) {
        if self.carret.index > index {
            let col = self.carret.col;
            self.set_index(self.carret.index - delta);
            // Update virtal column position only if the real column position changed
            if col!=self.carret.col {
                self.carret.vcol = col;
            }
        }

        if self.carret.selection > index {
            self.carret.selection -= delta;
        }
    }
}


#[derive(Debug, PartialEq, Eq)]
pub struct Carret {
    index: AbsoluteIndex,
    col: Column,
    pub line: usize,
    vcol: Column,
    pub rel_index: RelativeIndex,
    selection: AbsoluteIndex,
    pub is_clone: bool,
}

impl Clone for Carret {
    fn clone(&self) -> Self { 
        Self {
            index: self.index,
            col: self.col,
            vcol: self.vcol,
            line: self.line,
            rel_index: self.rel_index,
            selection: self.selection,
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
            vcol: Default::default(),
            col: Default::default(),
            line: Default::default(),
            rel_index: Default::default(),
            selection: Default::default(),
            is_clone: false,
        }
    }

    pub fn merge(c1: &Carret, c2: &Carret) -> Self {
        if c1.index < c1.selection {
            let (cstart, cend) = if c1.start() < c2.start() { (c1, c2) } else { (c2, c1) };
            Self {
                index: cstart.index,
                vcol: cstart.vcol,
                col: cstart.col,
                rel_index: cstart.rel_index,
                line: cstart.line,
                //col_index: cstart.col_index,
                selection: cend.end(),
                is_clone: cstart.is_clone && cend.is_clone,
            }
        } else {
            let (cstart, cend) = if c1.end() < c2.end() { (c1, c2) } else { (c2, c1) };
            Self {
                index: cend.end(),
                vcol: cend.vcol,
                col: cend.col,
                rel_index: cend.rel_index,
                line: cend.line,
                //col_index: cend.col_index,
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

    pub fn range(&self) -> Range<AbsoluteIndex> {
        if self.selection < self.index {
            return self.selection..self.index;
        } else {
            return self.index..self.selection;
        }
    }

    pub fn start(&self) -> AbsoluteIndex {
        self.range().start
    }

    pub fn end(&self) -> AbsoluteIndex {
        self.range().end
    }

   
    
    

    
}
