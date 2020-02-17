use crate::carret::Carrets;
use std::cell::RefCell;
use std::rc::Rc;
use std::io::Result;
use std::ops::{AddAssign, Range, RangeFrom, RangeFull, RangeTo, RangeInclusive};
use std::path::Path;

use ropey::{Rope, RopeSlice};
use unicode_segmentation::{GraphemeCursor, GraphemeIncomplete};

use crate::carret::Carret;
use crate::file::{Indentation, TextFileInfo};
use crate::rope_utils::*;

// fn collapse_selections(carrets: &mut Vec<CarretMut>) {
//     if carrets.len() > 1 {
//         carrets.sort_unstable_by(|a, b| a.carret.range().start.cmp(&b.carret.range().start))
//     }
//     let mut redo = true;
//     'outer: while redo {
//         for i in 0..carrets.len() - 1 {
//             if carrets[i].carret.collide_with(&carrets[i + 1].carret) {
//                 carrets[i].carret = Carret::merge(&carrets[i].carret, &carrets[i + 1].carret);
//                 carrets.remove(i + 1);
//                 redo = true;
//                 continue 'outer;
//             }
//         }
//         redo = false;
//     }
// }

// #[derive(Debug)]
// pub enum InvisibleChar {
//     Space(usize),
//     Tab(Range<usize>),
//     LineFeed(usize),
// }

#[derive(Debug, Default)]
pub struct EditStack {
    pub buffer: Buffer,
    undo_stack: Vec<Buffer>,
    redo_stack: Vec<Buffer>,
    pub file: TextFileInfo,
}

impl EditStack {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from_file<'a, P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = TextFileInfo::load(path)?;
        let buffer = Buffer::from_rope(file.1.clone(),Rc::new(file.0.indentation.visible_len()));
        Ok(Self {
            buffer,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            file: file.0,
        })
    }

    pub fn save(&mut self) -> Result<()> {
        self.file.save(&self.buffer.rope.borrow())?;
        Ok(())
    }
    pub fn save_as<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        self.file.save_as(&self.buffer.rope.borrow(), path)?;
        Ok(())
    }

    pub fn undo(&mut self) {
        if let Some(buffer) = self.undo_stack.pop() {
            let b = std::mem::take(&mut self.buffer);
            self.redo_stack.push(b);
            self.buffer = buffer;
        }
    }

    pub fn redo(&mut self) {
        if let Some(buffer) = self.redo_stack.pop() {
            let b = std::mem::take(&mut self.buffer);
            self.undo_stack.push(b);
            self.buffer = buffer;
        }
    }

    fn push_edit(&mut self, buffer: Buffer) {
        let b = std::mem::take(&mut self.buffer);
        self.undo_stack.push(b);
        self.buffer = buffer;
        self.redo_stack.clear();
    }

    pub fn forward(&mut self, expand_selection: bool) {
        self.buffer = self.buffer.forward(expand_selection);
    }
    pub fn backward(&mut self, expand_selection: bool) {
        self.buffer = self.buffer.backward(expand_selection);
    }
    pub fn up(&mut self, expand_selection: bool) {
        self.buffer = self.buffer.up(expand_selection);
    }
    pub fn down(&mut self, expand_selection: bool) {
        self.buffer = self.buffer.down(expand_selection);
    }
    pub fn duplicate_down(&mut self) {
        self.buffer = self.buffer.duplicate_down();
    }
    pub fn duplicate_up(&mut self) {
        self.buffer = self.buffer.duplicate_up();
    }
    pub fn revert_to_single_carrets(&mut self) {
        if self.buffer.carrets.len() > 1 {
            self.buffer = self.buffer.revert_to_single_carrets();
        }
    }
    pub fn cancel_selection(&mut self) {
        self.buffer = self.buffer.cancel_selection();
    }
    pub fn have_selection(&self) -> bool {
        self.buffer.have_selection()
    }
    pub fn home(&mut self, expand_selection: bool) {
        self.buffer = self.buffer.home(expand_selection);
    }
    pub fn end(&mut self, expand_selection: bool) {
        self.buffer = self.buffer.end(expand_selection);
    }
    pub fn insert(&mut self, text: &str) {
        let b = self.buffer.insert(text);
        self.push_edit(b);
    }
    pub fn backspace(&mut self) {
        if let Some(b) = self.buffer.backspace() {
            self.push_edit(b)
        };
    }
    pub fn delete(&mut self) {
        if let Some(b) = self.buffer.delete() {
            self.push_edit(b)
        };
    }
    pub fn tab(&mut self) {
        let b = self.buffer.tab(self.file.indentation);
        self.push_edit(b);
    }
}

#[derive(Debug, Clone)]
pub enum SelectionLineRange {
    Range(Range<usize>),
    RangeTo(RangeTo<usize>),
    RangeFrom(RangeFrom<usize>),
    RangeFull,
}

#[derive(Debug)]
pub struct Buffer {
    pub rope: Rc<RefCell<Rope>>,
    pub carrets: Carrets,
    tabsize: Rc<usize>,
}

impl Clone for Buffer {
    fn clone(&self) -> Self { 
        let rope = self.rope.borrow().clone();
        Self {
            rope: Rc::new(RefCell::new(rope)),
            carrets: self.carrets.clone(),
            tabsize: self.tabsize.clone(),
        }
    }
}

impl Default for Buffer {
    fn default() -> Self {
        let indentation = Indentation::default();
        Self::new(Rc::new(indentation.visible_len()))
    }
}

impl Buffer {
    pub fn new(tabsize: Rc<usize>) -> Self {
        let rope = Rc::new(RefCell::new(Rope::new()));
        Self {
            rope: rope.clone(),
            carrets: Carrets::new(rope.clone(),tabsize.clone()),
            tabsize: tabsize.clone(),
        }
    }

    pub fn carrets_on_line<'a>(&'a self, line_idx: usize) -> impl Iterator<Item = &'a Carret> {
        self.carrets
            .iter()
            .filter(move |c| c.line == line_idx)
    }

    // pub fn selection_index_on_line<'a>(&'a self, line_idx: usize) -> impl Iterator<Item = &'a Carret> {
    //     self.carrets.iter().filter(move |c| {
    //         if let Some(sel) = c.selection {
    //             self.rope.byte_to_line(sel) == line_idx
    //         } else {
    //             false
    //         }
    //     })
    // }

    pub fn selection_on_line<'a>(&'a self, line_idx: usize, ranges: &mut Vec<SelectionLineRange>) {
        ranges.clear();
        for r in self.carrets.iter().filter_map(move |c| {
            if !c.selection_is_empty() {
                let r = c.range();
                match (self.rope.borrow().byte_to_line(r.start.0), self.rope.borrow().byte_to_line(r.end.0)) {
                    (s, e) if s == e && s == line_idx => Some(SelectionLineRange::Range(
                        self.byte_to_line_relative_index(r.start.0)..self.byte_to_line_relative_index(r.end.0),
                    )),
                    (s, _) if s == line_idx => Some(SelectionLineRange::RangeFrom(
                        self.byte_to_line_relative_index(r.start.0)..,
                    )),
                    (_, e) if e == line_idx => {
                        Some(SelectionLineRange::RangeTo(..self.byte_to_line_relative_index(r.end.0)))
                    }
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

    pub fn byte_to_line_relative_index(&self, index: usize) -> usize {
        index - self.rope.borrow().line_to_byte(self.rope.borrow().byte_to_line(index))
    }

    pub fn byte_to_line(&self, index: usize) -> usize {
        self.rope.borrow().byte_to_line(index)
    }

    pub fn byte_to_line_range(&self, range: Range<AbsoluteIndex>) -> RangeInclusive<usize> {
        let line_start = self.rope.borrow().byte_to_line(range.start.0);
        let line_end = self.rope.borrow().byte_to_line(range.end.0);
        if self.byte_to_line_relative_index(range.end.0) == 0 && line_start!=line_end {
            line_start..=line_end - 1
        } else {
            line_start..=line_end
        }
    }

    pub fn from_rope(rope: Rope, tabsize: Rc<usize>) -> Self {
        let rope = Rc::new(RefCell::new(rope.clone()));
        Self {
            rope: rope.clone(),
            carrets: Carrets::new(rope.clone(),tabsize.clone()),
            tabsize: tabsize.clone(),
        }
    }

    pub fn line(&self, line: usize, out: &mut String) {
        out.clear();
        if line < self.rope.borrow().len_lines() {
            for r in self.rope.borrow().line(line).chunks() {
                out.push_str(r);
            }
        }
    }

    /// Construct a string with tab replaced as space
    /// return the position of invisible char
    pub fn displayable_line(&self, line: usize, tabsize: usize, out: &mut String, indices: &mut Vec<usize>) {
        out.clear();
        indices.clear();
        if line >= self.rope.borrow().len_lines() {
            return;
        }

        let mut index = 0;
        for c in self.rope.borrow().line(line).chars() {
            match c {
                ' ' => {
                    indices.push(index);
                    out.push(' ');
                    index += 1;
                }
                '\t' => {
                    let nb_space = tabsize - index % tabsize;
                    indices.push(index);
                    out.push_str(&" ".repeat(nb_space));
                    index += nb_space;
                }
                _ => {
                    out.push(c);
                    for i in index..index + c.len_utf8() {
                        indices.push(index);
                    }
                    index += c.len_utf8();
                }
            }
        }
        indices.push(index);
    }

    pub fn backward(&self, expand_selection: bool) -> Self {
        let mut buf = self.clone();
        for s in &mut buf.carrets.iter_mut() { 
            s.move_backward(expand_selection);
        }
        //collapse_selections(&mut carrets);
        buf.carrets.merge();
        buf
    }

    pub fn forward(&self, expand_selection: bool) -> Self {
        let mut buf = self.clone();
        for s in &mut buf.carrets.iter_mut() {
            s.move_forward(expand_selection);
        }
        //collapse_selections(&mut carrets);
        buf.carrets.merge();
        buf
    }

    pub fn up(&self, expand_selection: bool) -> Self {
        let mut buf = self.clone();
        for s in &mut buf.carrets.iter_mut() {
            s.move_up(expand_selection);
        }
        //collapse_selections(&mut carrets);
        buf.carrets.merge();
        buf
    }
    pub fn down(&self, expand_selection: bool) -> Self {
        let mut buf = self.clone();
        for s in &mut buf.carrets.iter_mut() {
            s.move_down(expand_selection);
        }
        //collapse_selections(&mut carrets);
        buf.carrets.merge();
        buf
    }
    pub fn duplicate_down(&self) -> Self {
        let mut buf = self.clone();
        buf.carrets.sort_unstable();//_by(|a, b| a.index.cmp(&b.index));

        if let Some(c) = buf.carrets.last().and_then(|c| c.duplicate_down()) {
            buf.carrets.push(c);
        }
        buf.carrets.merge();
        buf
    }

    pub fn duplicate_up(&self) -> Self {
        let mut buf = self.clone();
        buf.carrets.sort_unstable();//_by(|a, b| a.index.cmp(&b.index));

        if let Some(c) = buf.carrets.first().and_then(|c| c.duplicate_up()) {
            buf.carrets.push(c);
        }
        buf.carrets.merge();
        buf
    }

    pub fn cancel_selection(&self) -> Self {
        let mut buf = self.clone();
        for c in &mut buf.carrets.iter_mut() {
            //c.selection = None;
            c.cancel_selection();
        }
        buf
    }

    pub fn have_selection(&self) -> bool {
        self.carrets.iter().any(|c| !c.selection_is_empty())
    }

    pub fn revert_to_single_carrets(&self) -> Self {
        let mut buf = self.clone();
        buf.carrets.retain(|c| !c.is_clone);
        buf.carrets.merge();
        buf
    }

    pub fn end(&self, expand_selection: bool) -> Self {
        let mut buf = self.clone();
        for s in &mut buf.carrets.iter_mut() {

            s.move_end(expand_selection);
        }
        //collapse_selections(&mut carrets);
        buf.carrets.merge();
        buf
    }

    pub fn home(&self, expand_selection: bool) -> Self {
        let mut buf = self.clone();
        for s in &mut buf.carrets.iter_mut() {
            s.move_home(expand_selection);
        }
        //collapse_selections(&mut carrets);
        buf.carrets.merge();
        buf
    }

    pub fn insert(&self, text: &str) -> Self {

        let mut buf = self.clone();
        //carrets.sort_unstable_by(|a, b| a.index.cmp(&b.index));
        for i in 0..buf.carrets.len() {
            let r = buf.carrets[i].range();
            //buf.carrets[i].index = r.start;
            let insert_index = buf.rope.borrow().byte_to_char(r.start.0);
            let end_index = buf.rope.borrow().byte_to_char(r.end.0);
            let cr = insert_index..end_index;
            buf.rope.borrow_mut().remove(cr);
            buf.rope.borrow_mut().insert(insert_index, text);

            //
            buf.carrets[i].set_index(r.start + text.len(),true); // assume text have the correct grapheme boundary

            for j in i + 1..buf.carrets.len() {
                buf.carrets[j].update_after_delete(r.start, r.end.0 - r.start.0); // TODO verify this
                buf.carrets[j].update_after_insert(r.start, text.len());
                // if let Some(ref mut sel) = buf.carrets[j].selection {
                //     *sel -= r.end - r.start;
                //     *sel += text.len();
                // }
            }

            // let (vcol, line) = index_to_point(&rope.slice(..), buf.carrets[i].index);
            // buf.carrets[i].vcol = vcol;
            // buf.carrets[i].col_index = buf.carrets[i].index - rope.line_to_byte(line);
        }
        buf.carrets.merge();
        buf
        //Self { rope, carrets, tabsize: self.tabsize.clone() }
    }

    pub fn backspace(&self) -> Option<Self> {
        let mut buf = self.clone();

        let mut did_nothing = true;
        for i in 0..buf.carrets.len() {
            
            if !buf.carrets[i].selection_is_empty() {
                // delete selection
                dbg!(&buf.carrets[i]);
                let r = buf.carrets[i].range();
                //buf.carrets[i].selection = Default::default();
                // buf.carrets[i].index = r.start;
                buf.carrets[i].set_index(r.start,true);
                let rc = buf.rope.borrow().byte_to_char(r.start.0)..buf.rope.borrow().byte_to_char(r.end.0);
                buf.rope.borrow_mut().remove(rc);

                // update all others cursors
                for j in i + 1..buf.carrets.len() {
                    buf.carrets[j].update_after_delete(r.start, r.end.0 - r.start.0,);
                    // TODO verify this
                    // if let Some(ref mut sel) = buf.carrets[j].selection {
                    //     *sel -= r.end - r.start;
                    // }
                }

                did_nothing = false;
            } else if buf.carrets[i].index.0 > 0 {
                
                // delete the preceding grapheme
                let r = prev_grapheme_boundary(&buf.rope.borrow().slice(..), buf.carrets[i].index.0)..buf.carrets[i].index.0;
                buf.carrets[i].set_index(AbsoluteIndex(r.start),true);
                let cr = buf.rope.borrow().byte_to_char(r.start)..buf.rope.borrow().byte_to_char(r.end);
                buf.rope.borrow_mut().remove(cr);

                // update all others cursors
                for j in i + 1..buf.carrets.len() {
                    buf.carrets[j].update_after_delete(AbsoluteIndex(r.start), r.end - r.start);
                }
                did_nothing = false;
            } else {
                continue;
            }
            // let (vcol, line) = index_to_point(&rope.slice(..), buf.carrets[i].index);
            // buf.carrets[i].vcol = vcol;
            // buf.carrets[i].col_index = buf.carrets[i].index - rope.line_to_byte(line);
        }
        if did_nothing {
            None
        } else {
            buf.carrets.merge();
            Some(buf)
        }
    }

    pub fn delete(&self) -> Option<Self> {
        let mut buf = self.clone();
        let mut did_nothing = true;
        for i in 0..buf.carrets.len() {
            if !buf.carrets[i].selection_is_empty() {
                let r = buf.carrets[i].range();
                //buf.carrets[i].selection = Default::default();
                buf.carrets[i].set_index(r.start,true);
                let cr = buf.rope.borrow().byte_to_char(r.start.0)..buf.rope.borrow().byte_to_char(r.end.0);
                buf.rope.borrow_mut().remove(cr);

                // update all others cursors
                for j in i + 1..buf.carrets.len() {
                    buf.carrets[j].update_after_delete(r.start, r.end.0 - r.start.0);
                    // TODO verify this
                    // if let Some(ref mut sel) = carrets[j].selection {
                    //     *sel -= r.end - r.start;
                    // }
                }

                did_nothing = false;
            } else if buf.carrets[i].index.0 < buf.rope.borrow().len_bytes() {
                let r = buf.carrets[i].index.0..next_grapheme_boundary(&buf.rope.borrow().slice(..), buf.carrets[i].index.0);
                buf.carrets[i].set_index(AbsoluteIndex(r.start),true);
                let cr = buf.rope.borrow().byte_to_char(r.start)..buf.rope.borrow().byte_to_char(r.end);
                buf.rope.borrow_mut().remove(cr);
                // update all others cursors
                for j in i + 1..buf.carrets.len() {
                    buf.carrets[j].update_after_delete(AbsoluteIndex(r.start), r.end - r.start);
                }
                did_nothing = false;
            } else {
                continue;
            }
            // let (vcol, line) = index_to_point(&rope.slice(..), buf.carrets[i].index);
            // buf.carrets[i].vcol = vcol;
            // buf.carrets[i].col_index = buf.carrets[i].index - rope.line_to_byte(line);
        }
        if did_nothing {
            None
        } else {
            buf.carrets.merge();
            Some(buf)
        }
    }

    pub fn tab(&self, indentation: crate::file::Indentation) -> Self {
        let mut buf = self.clone();

        for i in 0..buf.carrets.len() {
            if let Some(line_range) = buf.carrets[i].selected_lines_range() {
                for line in line_range {
                    let inserted_byte: usize;
                    let line_char = buf.rope.borrow().line_to_char(line);
                    let line_byte = buf.rope.borrow().line_to_byte(line);
                    match indentation {
                        Indentation::Space(n) => {
                            let start = line_indent_len(&buf.rope.borrow().slice(..), line, n);
                            let nb_space = n - start % n;
                            buf.rope.borrow_mut().insert(line_char, &" ".repeat(nb_space));
                            inserted_byte = nb_space;
                        }
                        Indentation::Tab(_) => {
                            buf.rope.borrow_mut().insert_char(line_char, '\t');
                            inserted_byte = 1;
                        }
                    }
                    for j in i..buf.carrets.len() {
                        buf.carrets[j].update_after_insert(AbsoluteIndex(line_byte), inserted_byte);
                    }
                }
            } else {
                let r = buf.carrets[i].range();
                let text = match indentation {
                    Indentation::Space(n) => {
                        // let i = r.start - rope.line_to_byte(*line_range.start());
                        // let start = line_index_to_column(&rope.line(*line_range.start()),i,n);
                        let start = buf.carrets[i].column_index();
                        let nb_space = n - start.0 % n;
                        " ".repeat(nb_space).to_owned()
                    }
                    Indentation::Tab(_) => "\t".to_owned(),
                };

                //carrets[i].index = r.start;
                let cr_start = buf.rope.borrow().byte_to_char(r.start.0);
                let cr_end = buf.rope.borrow().byte_to_char(r.end.0);
                buf.rope.borrow_mut().remove(cr_start..cr_end);
                buf.rope.borrow_mut().insert(cr_start, &text);

                //carrets[i].selection = Default::default();
                buf.carrets[i].set_index(r.start + text.len(),true); // assume text have the correct grapheme boundary
                for j in i + 1..buf.carrets.len() {
                    buf.carrets[j].update_after_delete(r.start, r.end.0 - r.start.0); // TODO verify this
                    buf.carrets[j].update_after_insert(r.start, text.len());
                    // if let Some(ref mut sel) = buf.carrets[j].selection {
                    //     *sel -= r.end - r.start;
                    //     *sel += text.len();
                    // }
                }
            }
        }
        buf.carrets.merge();
        buf
    }

    //pub fn tab(&self,tab_size: usize) -> Self {

    //     if self.carrets.len()>1 {
    //         let nb_space
    //         self.insert()
    //     } else {

    //     }
    // }
}

impl ToString for Buffer {
    fn to_string(&self) -> String {
        self.rope.borrow().to_string()
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     #[test]
//     fn rope_insert() {
//         let b = Buffer::new();
//         assert_eq!(b.insert("hello world").to_string(), "hello world");
//     }
//     #[test]
//     fn rope_double_insert() {
//         let b = Buffer::new();
//         println!("{:?}", b.insert("hello"));
//         assert_eq!(b.insert("hello").insert(" world").to_string(), "hello world");
//     }
//     #[test]
//     fn rope_backspace() {
//         let b = Buffer::new();
//         assert_eq!(b.insert("hello").backspace().unwrap().to_string(), "hell");
//     }
//     #[test]
//     fn rope_backspace2() {
//         let b = Buffer::new();
//         assert_eq!(b.insert("").backspace().unwrap().to_string(), "");
//     }
//     #[test]
//     fn rope_right() {
//         let b = Buffer::new();
//         let mut b = b.insert("hello\n");
//         b.carrets[0].index = 0;
//         let b = b.forward(false);
//         assert_eq!(b.carrets[0].index, 1);

//         let b = b.forward(false).forward(false).forward(false);
//         assert_eq!(b.carrets[0].index, 4);
//         // move 3 forward, but the last move is ineffective because beyond the rope lenght
//         let b = b.forward(false).forward(false).forward(false);
//         assert_eq!(b.carrets[0].index, 6);
//     }
//     // #[test]
//     // fn rope_forward() {
//     //     let indexes = vec![1usize, 2, 3, 4, 5, 7, 8, 9, 10, 11, 12, 12];
//     //     let mut b = Buffer::new().insert("hello\r\nWorld");
//     //     b.carrets[0].index = 0;

//     //     for i in &indexes {
//     //         b = b.forward(false);
//     //         assert_eq!(b.carrets[0].index, *i);
//     //     }
//     //     b.carrets[0].index = 0;

//     //     for i in &indexes {
//     //         b = b.forward(true);
//     //         assert_eq!(b.carrets[0].selection.byte_len, *i);
//     //         assert_eq!(
//     //             b.carrets[0].selection.direction,
//     //             SelectionDirection::Backward
//     //         );
//     //     }
//     // }
//     // #[test]
//     // fn rope_backward() {
//     //     let indexes = vec![11, 10, 9, 8, 7, 5, 4, 3, 2, 1, 0, 0];
//     //     let mut b = Buffer::new().insert("hello\r\nWorld");

//     //     for i in &indexes {
//     //         b = b.backward(false);
//     //         assert_eq!(b.carrets[0].index, *i);
//     //     }
//     //     let mut b = Buffer::new().insert("hello\r\nWorld");
//     //     let len = vec![1, 2, 3, 4, 5, 7, 8, 9, 10, 11, 12, 12];
//     //     for i in &len {
//     //         b = b.backward(true);
//     //         assert_eq!(b.carrets[0].selection.byte_len, *i);
//     //         assert_eq!(
//     //             b.carrets[0].selection.direction,
//     //             SelectionDirection::Forward
//     //         );
//     //     }
//     // }
// }
