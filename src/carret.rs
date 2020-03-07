use crate::rope_utils::*;
use crate::rope_utils::{AbsoluteIndex, Column, RelativeIndex};
use ropey::{Rope, RopeSlice};
use std::cell::RefCell;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ops::{AddAssign, Range, RangeInclusive, SubAssign};
use std::rc::Rc;

#[derive(Debug)]
pub struct Carrets {
    intern: Vec<Carret>,
}

impl Carrets {
    pub fn new() -> Self {
        let mut intern = Vec::new();
        intern.push(Carret::new());
        Self { intern }
    }
    // pub fn get_mut(self, rope: &Rope, tabsize: usize) -> CarretsMut {
    //     CarretsMut {
    //         intern: self.intern.iter().map(|c| c.clone().get_mut(rope, tabsize)).collect(),
    //     }
    // }
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
        let mut intern: Vec::<Carret> = self.intern.iter().filter(|c| c.is_clone).map(|c| c.clone()).collect();
        let mut first = self.intern.iter().filter(|c| !c.is_clone).nth(0).unwrap().clone();
        first.is_clone=false;
        intern.push(first);
        intern.sort_unstable();
        Self { intern }
    }
}

// impl Default for Carrets {
//     fn default() -> Self {
//         let mut intern = Vec::new();
//         intern.push(Carret::new());
//         Self { intern }
//     }
// }

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

// #[derive(Debug, Clone)]
// pub struct CarretsMut<'rs> {
//     intern: Vec<CarretMut<'rs>>,
// }

// impl<'rs> CarretsMut<'rs> {
//     pub fn fix(mut self) -> Carrets {
//         self.merge();
//         Carrets {
//             intern: self.intern.iter().map(|c| c.clone().carret).collect(),
//         }
//     }

//     fn merge(&mut self) {
//         if self.intern.len() > 1 {
//             self.intern
//                 .sort_unstable_by(|a, b| a.carret.range().start.cmp(&b.carret.range().start))
//         }
//         let mut redo = true;
//         'outer: while redo {
//             for i in 0..self.intern.len() - 1 {
//                 if self.intern[i].carret.collide_with(&self.intern[i + 1].carret) {
//                     self.intern[i].carret = Carret::merge(&self.intern[i].carret, &self.intern[i + 1].carret);
//                     self.intern.remove(i + 1);
//                     redo = true;
//                     continue 'outer;
//                 }
//             }
//             redo = false;
//         }
//     }
// }

// impl<'rs> Deref for CarretsMut<'rs> {
//     type Target = Vec<CarretMut<'rs>>;
//     fn deref(&self) -> &Self::Target {
//         &self.intern
//     }
// }

// impl<'rs> DerefMut for CarretsMut<'rs> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.intern
//     }
// }

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct CarretMut<'rs> {
//     pub carret: Carret,
//     owner: RopeSlice<'rs>,
//     tabsize: usize,
// }

// impl<'rs> PartialOrd for CarretMut<'rs> {
//     fn partial_cmp(&self, other: &CarretMut) -> std::option::Option<std::cmp::Ordering> {
//         self.partial_cmp(&other.carret)
//     }
// }

// impl<'rs> Ord for CarretMut<'rs> {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         self.cmp(&other.carret)
//     }
// }

// impl<'rs> CarretMut<'rs> {

// }

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Carret {
    pub index: AbsoluteIndex,
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
        self.selection == self.index
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

    pub fn start_line(&self,rope: &Rope) -> usize {
        rope.byte_to_line(self.start().0)
    }

    pub fn end_line(&self,rope: &Rope) -> usize {
        rope.byte_to_line(self.end().0)
    }

    pub fn selected_lines_range(&self,rope: &Rope) -> Option<RangeInclusive<usize>> {
        if self.selection_is_empty() {
            None
        } else {
            Some(self.start_line(rope)..=self.end_line(rope))
        }
    }

    pub fn line(&self,rope: &Rope) -> Line {
        //rope.byte_to_line(self.index)
        Line::for_index(self.index, rope)
    }

    pub fn relative_index(&self,rope: &Rope) -> RelativeIndex {
        self.line(rope).absolute_to_relative_index(self.index,rope)
    }

    pub fn column_index(&self,rope: &Rope, tabsize: usize) -> Column {
        self.line(rope)
            .relative_index_to_column(self.relative_index(rope), rope, tabsize)
    }

    fn recalculate_vcol(&mut self,rope: &Rope,tabsize: usize) {
        self.vcol = self.column_index(rope,tabsize);
    }

    pub fn set_index(&mut self, index: AbsoluteIndex, reset_selection: bool,rope: &Rope,tabsize: usize) {
        self.index = index;
        if reset_selection {
            self.selection = index;
        }
        self.col = self.column_index(rope, tabsize);
        //let line = self.line();
        self.rel_index = self.line(rope).absolute_to_relative_index(self.index,rope);
        self.line = self.line(rope).line;
    }

    pub fn move_up(&mut self, expand_selection: bool,rope: &Rope,tabsize: usize) {
        //let line = rope.byte_to_line(s.index);
        if let Some(prev_line) = self.line(rope).prev_line() {
            let index = prev_line.column_to_absolute_index(self.vcol, rope, tabsize);
            self.set_index(index, !expand_selection,rope,tabsize);
        }
    }

    pub fn move_down(&mut self, expand_selection: bool,rope: &Rope,tabsize: usize) {
        //let line = rope.byte_to_line(s.index);
        if let Some(next_line) = self.line(rope).next_line(rope) {
            let index = next_line.column_to_absolute_index(self.vcol,rope, tabsize);
            self.set_index(index, !expand_selection,rope,tabsize);
        }
    }

    pub fn move_backward(&mut self, expand_selection: bool,rope: &Rope,tabsize: usize) {
        let index = prev_grapheme_boundary(&rope.slice(..), self.index.0);
        self.set_index(AbsoluteIndex(index), !expand_selection,rope,tabsize);
        self.recalculate_vcol(rope,tabsize);
    }

    pub fn move_forward(&mut self, expand_selection: bool,rope: &Rope,tabsize: usize) {
        let index = next_grapheme_boundary(&rope.slice(..), self.index.0);
        self.set_index(AbsoluteIndex(index), !expand_selection,rope,tabsize);
        self.recalculate_vcol(rope,tabsize);
    }

    pub fn duplicate_down(&self,rope: &Rope,tabsize: usize) -> Option<Self> {
        if let Some(next_line) = self.line(rope).next_line(rope) {
            let mut c = self.clone();
            c.set_index(next_line.column_to_absolute_index(self.vcol, rope, tabsize), true,rope, tabsize);
            Some(c)
        } else {
            None
        }
    }

    pub fn duplicate_up(&self,rope: &Rope,tabsize: usize) -> Option<Self> {
        if let Some(prev_line) = self.line(rope).prev_line() {
            let mut c = self.clone();
            c.set_index(prev_line.column_to_absolute_index(self.vcol, rope, tabsize), true,rope,tabsize);
            Some(c)
        } else {
            None
        }
    }

    pub fn move_end(&mut self, expand_selection: bool,rope: &Rope,tabsize: usize) {
        let index = self.line(rope).end(rope);
        self.set_index(index, !expand_selection,rope,tabsize);
        self.recalculate_vcol(rope,tabsize);
    }

    pub fn move_home(&mut self, expand_selection: bool,rope: &Rope,tabsize: usize) {
        let index = self.line(rope).start(rope);
        self.set_index(index, !expand_selection,rope,tabsize);
        self.recalculate_vcol(rope,tabsize);
    }

    pub fn update_after_insert(&mut self, index: AbsoluteIndex, delta: usize,rope: &Rope,tabsize: usize) {
        if self.index > index {
            let col = self.col;
            self.set_index(self.index + delta, false,rope,tabsize);
            // Update virtal column position only if the real column position changed
            if col != self.col {
                self.vcol = col;
            }
        }
        if self.selection > index {
            self.selection += delta;
        }
    }
    pub fn update_after_delete(&mut self, index: AbsoluteIndex, delta: usize,rope: &Rope,tabsize: usize) {
        if self.index > index {
            let col = self.col;
            self.set_index(self.index - delta, false,rope,tabsize);
            // Update virtal column position only if the real column position changed
            if col != self.col {
                self.vcol = col;
            }
        }

        if self.selection > index {
            self.selection -= delta;
        }
    }
}
