use std::cell::RefCell;
use std::rc::Rc;
use crate::rope_utils::*;
use crate::rope_utils::{AbsoluteIndex, Column, RelativeIndex};
use ropey::{Rope, RopeSlice};
use std::ops::Deref;
use std::ops::DerefMut;
use std::ops::{AddAssign, Range, RangeInclusive, SubAssign};

#[derive(Debug, Clone)]
pub struct Carrets {
    intern: Vec<Carret>,
}

impl Carrets {
    pub fn new(rope: Rc<RefCell<Rope>>, tabsize: Rc<usize>) -> Self{
        let mut intern = Vec::new();
        intern.push(Carret::new(rope,tabsize));
        Self { intern }
    }
    // pub fn get_mut(self, rope: &Rope, tabsize: usize) -> CarretsMut {
    //     CarretsMut {
    //         intern: self.intern.iter().map(|c| c.clone().get_mut(rope, tabsize)).collect(),
    //     }
    // }
    fn merge(&mut self) {
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
    owner: Rc<RefCell<Rope>>,
    tabsize: Rc<usize>,
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
            owner: self.owner.clone(),
            tabsize: self.tabsize.clone(),
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
    pub fn new(owner : Rc<RefCell<Rope>>, tabsize: Rc<usize>) -> Self {
        Self {
            owner: owner.clone(),
            tabsize: tabsize.clone(),
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
                owner: cstart.owner.clone(),
                tabsize: cstart.tabsize.clone(),
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
                owner: cend.owner.clone(),
                tabsize: cend.tabsize.clone(),
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

    pub fn start_line(&self) -> usize {
        self.owner.borrow().byte_to_line(self.start().0)
    }

    pub fn end_line(&self) -> usize {
        self.owner.borrow().byte_to_line(self.end().0)
    }

    pub fn selected_lines_range(&self) -> Option<RangeInclusive<usize>> {
        if self.selection_is_empty() {
            None
        } else {
            Some(self.start_line()..=self.end_line())
        }
    }

    pub fn line(&self) -> Line {
        //rope.byte_to_line(self.index)
        Line::for_index(self.index, self.owner.clone())
    }

    pub fn relative_index(&self) -> RelativeIndex {
        self.line().absolute_to_relative_index(self.index)
    }

    pub fn column_index(&self) -> Column {
        self.line()
            .relative_index_to_column(self.relative_index(), *self.tabsize)
    }

    // fn recalculate_col_index(&mut self, rope: &Rope) {
    //     self.col_index = self.index - rope.line_to_byte(self.index_line(rope));
    // }

    fn recalculate_vcol(&mut self) {
        self.vcol = self.column_index();
    }

    pub fn set_index(&mut self, index: AbsoluteIndex, reset_selection: bool) {
        self.index = index;
        if reset_selection {
            self.selection = index;
        }
        self.col = self.column_index();
        //let line = self.line();
        self.rel_index = self.line().absolute_to_relative_index(self.index);
        self.line = self.line().line;
    }

    pub fn move_up(&mut self, expand_selection: bool) {
        //let line = rope.byte_to_line(s.index);
        if let Some(prev_line) = self.line().prev_line() {
            let index = prev_line.column_to_absolute_index(self.vcol, *self.tabsize);
            self.set_index(index,!expand_selection);
        }
    }

    pub fn move_down(&mut self, expand_selection: bool) {
        //let line = rope.byte_to_line(s.index);
        if let Some(next_line) = self.line().next_line() {
            let index = next_line.column_to_absolute_index(self.vcol, *self.tabsize);
            self.set_index(index,!expand_selection);
        }
    }

    pub fn move_backward(&mut self, expand_selection: bool) {
        let index = prev_grapheme_boundary(&self.owner.borrow().slice(..), self.index.0);
        self.set_index(AbsoluteIndex(index),!expand_selection);
        self.recalculate_vcol();
    }

    pub fn move_forward(&mut self, expand_selection: bool) {
        let index = next_grapheme_boundary(&self.owner.borrow().slice(..), self.index.0);
        self.set_index(AbsoluteIndex(index),!expand_selection);
        self.recalculate_vcol();
    }

    pub fn duplicate_down(&self) -> Option<Self> {
        if let Some(next_line) = self.line().next_line() {
            let mut c = self.clone();
            c.set_index(next_line.column_to_absolute_index(self.vcol, *self.tabsize),true);
            Some(c)
        } else {
            None
        }
    }

    pub fn duplicate_up(&self) -> Option<Self> {
        if let Some(prev_line) = self.line().prev_line() {
            let mut c = self.clone();
            c.set_index(prev_line.column_to_absolute_index(self.vcol, *self.tabsize),true);
            Some(c)
        } else {
            None
        }
    }

    pub fn move_end(&mut self, expand_selection: bool) {
        let index = self.line().end();
        self.set_index(index,!expand_selection);
        self.recalculate_vcol();
    }

    pub fn move_home(&mut self, expand_selection: bool) {
        let index = self.line().start();
        self.set_index(index,!expand_selection);
        self.recalculate_vcol();
    }

    pub fn update_after_insert(&mut self, index: AbsoluteIndex, delta: usize) {
        if self.index > index {
            let col = self.col;
            self.set_index(self.index + delta,false);
            // Update virtal column position only if the real column position changed
            if col != self.col {
                self.vcol = col;
            }
        }
        if self.selection > index {
            self.selection += delta;
        }
    }
    pub fn update_after_delete(&mut self, index: AbsoluteIndex, delta: usize) {
        if self.index > index {
            let col = self.col;
            self.set_index(self.index - delta,false);
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
