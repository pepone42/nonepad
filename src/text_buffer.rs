use std::io::Result;
use std::ops::{AddAssign, Range, RangeFrom, RangeFull, RangeTo};
use std::{path::Path, rc::Rc};

use ropey::{Rope, RopeSlice};
use unicode_segmentation::{GraphemeCursor, GraphemeIncomplete};

use crate::carret::Carret;
use crate::file::{Indentation, TextFileInfo};
use crate::rope_utils::*;

fn collapse_selections(carrets: &mut Vec<Carret>) {
    if carrets.len() > 1 {
        carrets.sort_unstable_by(|a, b| a.range().start.cmp(&b.range().start))
    }
    let mut redo = true;
    'outer: while redo {
        for i in 0..carrets.len() - 1 {
            if carrets[i].collide_with(&carrets[i + 1]) {
                carrets[i] = Carret::merge(&carrets[i], &carrets[i + 1]);
                carrets.remove(i + 1);
                redo = true;
                continue 'outer;
            }
        }
        redo = false;
    }
}

#[derive(Debug)]
pub enum InvisibleChar {
    Space(usize),
    Tab(Range<usize>),
    LineFeed(usize),
}

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
        let buffer = Buffer::from_rope(file.1.clone());
        Ok(Self {
            buffer,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            file: file.0,
        })
    }

    pub fn save(&mut self) -> Result<()> {
        self.file.save(&self.buffer.rope)?;
        Ok(())
    }
    pub fn save_as<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        self.file.save_as(&self.buffer.rope, path)?;
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
}

#[derive(Debug, Clone)]
pub enum SelectionLineRange {
    Range(Range<usize>),
    RangeTo(RangeTo<usize>),
    RangeFrom(RangeFrom<usize>),
    RangeFull,
}

#[derive(Debug, Clone)]
pub struct Buffer {
    pub rope: Rope,
    pub carrets: Vec<Carret>,
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}

impl Buffer {
    pub fn new() -> Self {
        let rope = Rope::new();
        Self {
            rope: rope.clone(),
            carrets: {
                let mut v = Vec::new();
                v.push(Carret::new());
                v
            },
        }
    }

    pub fn carrets_on_line<'a>(&'a self, line_idx: usize) -> impl Iterator<Item = &'a Carret> {
        self.carrets
            .iter()
            .filter(move |c| self.rope.byte_to_line(c.index) == line_idx)
    }

    pub fn selection_index_on_line<'a>(&'a self, line_idx: usize) -> impl Iterator<Item = &'a Carret> {
        self.carrets.iter().filter(move |c| {
            if let Some(sel) = c.selection {
                self.rope.byte_to_line(sel) == line_idx
            } else {
                false
            }
        })
    }

    pub fn selection_on_line<'a>(&'a self, line_idx: usize, ranges: &mut Vec<SelectionLineRange>) {
        ranges.clear();
        for r in self.carrets.iter().filter_map(move |c| {
            if let Some(sel) = c.selection {
                let r = c.range();
                match (self.rope.byte_to_line(r.start), self.rope.byte_to_line(r.end)) {
                    (s, e) if s == e && s == line_idx => Some(SelectionLineRange::Range(
                        self.byte_to_line_relative_index(r.start)..self.byte_to_line_relative_index(r.end),
                    )),
                    (s, _) if s == line_idx => Some(SelectionLineRange::RangeFrom(
                        self.byte_to_line_relative_index(r.start)..,
                    )),
                    (_, e) if e == line_idx => {
                        Some(SelectionLineRange::RangeTo(..self.byte_to_line_relative_index(r.end)))
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
        index - self.rope.line_to_byte(self.rope.byte_to_line(index))
    }

    pub fn byte_to_line(&self, index: usize) -> usize {
        self.rope.byte_to_line(index)
    }

    pub fn byte_to_line_range(&self, range: Range<usize>) -> Range<usize> {
        self.rope.byte_to_line(range.start)..self.rope.byte_to_line(range.end)
    }

    pub fn from_rope(rope: Rope) -> Self {
        Self {
            rope: rope.clone(),
            carrets: {
                let mut v = Vec::new();
                v.push(Carret::new());
                v
            },
        }
    }

    pub fn line(&self, line: usize, out: &mut String) {
        out.clear();
        if line < self.rope.len_lines() {
            for r in self.rope.line(line).chunks() {
                out.push_str(r);
            }
        }
    }

    /// Construct a string with tab replaced as space
    /// return the position of invisible char
    pub fn displayable_line(&self, line: usize, tabsize: usize, out: &mut String, indices: &mut Vec<usize>) {
        out.clear();
        indices.clear();
        if line >= self.rope.len_lines() {
            return;
        }

        let mut index = 0;
        for c in self.rope.line(line).chars() {
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
                    indices.push(index);
                    out.push(c);
                    index += c.len_utf8();
                }
            }
        }
        indices.push(index);
    }

    pub fn backward(&self, expand_selection: bool) -> Self {
        let rope = self.rope.clone();
        let mut carrets = self.carrets.clone();
        for s in &mut carrets {
            let index = prev_grapheme_boundary(&rope.slice(..), s.index);

            if expand_selection {
                if s.selection.is_none() {
                    s.selection = Some(s.index);
                }
            } else {
                s.selection = None;
            };
            s.set_index(index,&rope);
            // let (vcol, line) = index_to_point(&rope.slice(..), s.index);
            // s.vcol = vcol;
            // s.col_index = s.index - rope.line_to_byte(line);
        }
        collapse_selections(&mut carrets);
        Self { rope, carrets }
    }

    pub fn forward(&self, expand_selection: bool) -> Self {
        let rope = self.rope.clone();
        let mut carrets = self.carrets.clone();
        for s in &mut carrets {
            let index = next_grapheme_boundary(&rope.slice(..), s.index);

            if expand_selection {
                if s.selection.is_none() {
                    s.selection = Some(s.index);
                }
            } else {
                s.selection = None;
            };
            s.set_index(index,&rope);
            // s.index = index;
            // let (vcol, line) = index_to_point(&rope.slice(..), s.index);
            // s.vcol = vcol;
            // s.col_index = s.index - rope.line_to_byte(line);
        }
        collapse_selections(&mut carrets);
        Self { rope, carrets }
    }

    pub fn up(&self, expand_selection: bool) -> Self {
        let rope = self.rope.clone();
        let mut carrets = self.carrets.clone();
        for s in &mut carrets {
            s.move_up(expand_selection,&rope);
        }
        collapse_selections(&mut carrets);
        Self { rope, carrets }
    }
    pub fn down(&self, expand_selection: bool) -> Self {
        let rope = self.rope.clone();
        let mut carrets = self.carrets.clone();
        for s in &mut carrets {
            s.move_down(expand_selection,&rope);
        }
        collapse_selections(&mut carrets);
        Self { rope, carrets }
    }
    pub fn duplicate_down(&self) -> Self {
        let rope = self.rope.clone();
        let mut carrets = self.carrets.clone();
        carrets.sort_unstable_by(|a, b| a.index.cmp(&b.index));
        //let mut c = carrets.last().unwrap();

        // let line = rope.byte_to_line(c.index);
        // if line < self.rope.len_lines() - 1 {
        //     let (index, col_index, _) = point_to_index(&rope.slice(..), c.vcol, line + 1);

        //     c.col_index = col_index;
        //     c.index = index;
        //     c.is_clone = true;
        //     carrets.push(c);
        // }
        if let Some(c) = carrets.last().and_then(|c| c.duplicate_down(&rope)) {
            carrets.push(c);
        }
        Self { rope, carrets }
    }

    pub fn duplicate_up(&self) -> Self {
        let rope = self.rope.clone();
        let mut carrets = self.carrets.clone();
        carrets.sort_unstable_by(|a, b| a.index.cmp(&b.index));

        if let Some(c) = carrets.first().and_then(|c| c.duplicate_up(&rope)) {
            carrets.push(c);
        }
        Self { rope, carrets }
    }

    pub fn revert_to_single_carrets(&self) -> Self {
        let rope = self.rope.clone();
        let mut carrets = self.carrets.clone();
        carrets.retain(|c| !c.is_clone);
        Self { rope, carrets }
    }

    pub fn end(&self, expand_selection: bool) -> Self {
        let rope = self.rope.clone();
        let mut carrets = self.carrets.clone();
        for s in &mut carrets {
            let line = rope.byte_to_line(s.index);
            let line_boundary = line_boundary(&rope.slice(..), line);
            let index = line_boundary.end;

            if expand_selection {
                if s.selection.is_none() {
                    s.selection = Some(s.index);
                }
            } else {
                s.selection = None;
            };
            s.set_index(index,&rope);
            // let (vcol, _) = index_to_point(&rope.slice(..), s.index);
            // s.vcol = vcol;
            // s.col_index = s.index - line_boundary.start;
        }
        collapse_selections(&mut carrets);
        Self { rope, carrets }
    }

    pub fn home(&self, expand_selection: bool) -> Self {
        let rope = self.rope.clone();
        let mut carrets = self.carrets.clone();
        for s in &mut carrets {
            let line_boundary = byte_to_line_boundary(&rope.slice(..), s.index);
            let index = byte_to_line_first_column(&rope.slice(..), s.index);

            if expand_selection {
                if s.selection.is_none() {
                    s.selection = Some(s.index);
                }
            } else {
                s.selection = None;
            };
            s.set_index(index,&rope);
            // let (vcol, _) = index_to_point(&rope.slice(..), s.index);
            // s.vcol = vcol;
            // s.col_index = s.index - line_boundary.start;
        }
        collapse_selections(&mut carrets);
        Self { rope, carrets }
    }

    pub fn insert(&self, text: &str) -> Self {
        let mut rope = self.rope.clone();
        let mut carrets = self.carrets.clone();
        carrets.sort_unstable_by(|a, b| a.index.cmp(&b.index));
        for i in 0..carrets.len() {
            let r = carrets[i].range();
            carrets[i].index = r.start;
            rope.remove(rope.byte_to_char(r.start)..rope.byte_to_char(r.end));
            rope.insert(rope.byte_to_char(carrets[i].index), text);

            carrets[i].selection = Default::default();
            carrets[i].add(text.len(),&rope); // assume text have the correct grapheme boundary

            for j in i + 1..carrets.len() {
                carrets[j].sub(r.end - r.start,&rope); // TODO verify this
                carrets[j].add(text.len(),&rope);
                // if let Some(ref mut sel) = carrets[j].selection {
                //     *sel -= r.end - r.start;
                //     *sel += text.len();
                // }
            }

            
            // let (vcol, line) = index_to_point(&rope.slice(..), carrets[i].index);
            // carrets[i].vcol = vcol;
            // carrets[i].col_index = carrets[i].index - rope.line_to_byte(line);
        }
        Self { rope, carrets }
    }

    pub fn backspace(&self) -> Option<Self> {
        let mut rope = self.rope.clone();
        let mut carrets = self.carrets.clone();
        carrets.sort_unstable_by(|a, b| a.index.cmp(&b.index));

        let mut did_nothing = true;
        for i in 0..carrets.len() {
            if carrets[i].selection.is_some() {
                // delete selection
                
                let r = carrets[i].range();
                carrets[i].selection = Default::default();
                // carrets[i].index = r.start;
                carrets[i].set_index(r.start,&rope);
                rope.remove(rope.byte_to_char(r.start)..rope.byte_to_char(r.end));

                // update all others cursors
                for j in i + 1..carrets.len() {
                    carrets[j].sub(r.end - r.start,&rope); // TODO verify this
                    // if let Some(ref mut sel) = carrets[j].selection {
                    //     *sel -= r.end - r.start;
                    // }
                }
                
                did_nothing = false;
            } else if carrets[i].index > 0 {
                // delete the preceding grapheme
                let r = prev_grapheme_boundary(&rope.slice(..), carrets[i].index)..carrets[i].index;
                carrets[i].set_index(r.start,&rope);
                rope.remove(rope.byte_to_char(r.start)..rope.byte_to_char(r.end));

                // update all others cursors
                for j in i + 1..carrets.len() {
                    carrets[j].sub(r.end - r.start,&rope);
                }
                did_nothing = false;
            } else {
                continue;
            }
            // let (vcol, line) = index_to_point(&rope.slice(..), carrets[i].index);
            // carrets[i].vcol = vcol;
            // carrets[i].col_index = carrets[i].index - rope.line_to_byte(line);
        }
        if did_nothing {
            None
        } else {
            Some(Self { rope, carrets })
        }
    }

    pub fn delete(&self) -> Option<Self> {
        let mut rope = self.rope.clone();
        let mut carrets = self.carrets.clone();
        carrets.sort_unstable_by(|a, b| a.index.cmp(&b.index));
        let mut did_nothing = true;
        for i in 0..carrets.len() {
            if carrets[i].selection.is_some() {
                let r = carrets[i].range();
                carrets[i].selection = Default::default();
                carrets[i].set_index(r.start,&rope);
                rope.remove(rope.byte_to_char(r.start)..rope.byte_to_char(r.end));

                // update all others cursors
                for j in i + 1..carrets.len() {
                    carrets[j].sub(r.end - r.start,&rope); // TODO verify this
                    // if let Some(ref mut sel) = carrets[j].selection {
                    //     *sel -= r.end - r.start;
                    // }
                }
                
                did_nothing = false;
            } else if carrets[i].index < rope.len_bytes() - 1 {
                let r = carrets[i].index..next_grapheme_boundary(&rope.slice(..), carrets[i].index);
                carrets[i].set_index(r.start,&rope);
                rope.remove(rope.byte_to_char(r.start)..rope.byte_to_char(r.end));
                // update all others cursors
                for j in i + 1..carrets.len() {
                    carrets[j].sub(r.end - r.start,&rope);
                }
                did_nothing = false;
            } else {
                continue;
            }
            // let (vcol, line) = index_to_point(&rope.slice(..), carrets[i].index);
            // carrets[i].vcol = vcol;
            // carrets[i].col_index = carrets[i].index - rope.line_to_byte(line);
        }
        if did_nothing {
            None
        } else {
            Some(Self { rope, carrets })
        }
    }

    pub fn indent(&self, indentation: crate::file::Indentation) -> Self {
        assert_eq!(self.carrets.len(), 1);
        let mut rope = self.rope.clone();
        let mut carrets = self.carrets.clone();
        for i in self.byte_to_line_range(carrets[0].range()) {
            match indentation {
                Indentation::Space(n) => rope.insert(rope.line_to_char(i), &" ".repeat(n)),
                Indentation::Tab(_) => rope.insert(rope.line_to_char(i), "\t"),
            }
        }
        carrets[0].index += indentation.len_as_byte();
        if let Some(a) = carrets[0].selection.as_mut() {
            *a += indentation.len_as_byte();
        }
        Self { rope, carrets }
    }

    // pub fn tab(&self,tab_size: usize) -> Self {
    //     // 3 use case.
    //     // 1 cursor, no selection -> insert a tab (or some space)
    //     // 1 cursor, a selection -> the same for every selected line -> indent
    //     // n cursor, with or without selection -> insert tab and replace the selection
    //     if self.carrets.len()>1 {
    //         let nb_space
    //         self.insert()
    //     } else {

    //     }
    // }
}

impl ToString for Buffer {
    fn to_string(&self) -> String {
        self.rope.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn rope_insert() {
        let b = Buffer::new();
        assert_eq!(b.insert("hello world").to_string(), "hello world");
    }
    #[test]
    fn rope_double_insert() {
        let b = Buffer::new();
        println!("{:?}", b.insert("hello"));
        assert_eq!(b.insert("hello").insert(" world").to_string(), "hello world");
    }
    #[test]
    fn rope_backspace() {
        let b = Buffer::new();
        assert_eq!(b.insert("hello").backspace().unwrap().to_string(), "hell");
    }
    #[test]
    fn rope_backspace2() {
        let b = Buffer::new();
        assert_eq!(b.insert("").backspace().unwrap().to_string(), "");
    }
    #[test]
    fn rope_right() {
        let b = Buffer::new();
        let mut b = b.insert("hello\n");
        b.carrets[0].index = 0;
        let b = b.forward(false);
        assert_eq!(b.carrets[0].index, 1);

        let b = b.forward(false).forward(false).forward(false);
        assert_eq!(b.carrets[0].index, 4);
        // move 3 forward, but the last move is ineffective because beyond the rope lenght
        let b = b.forward(false).forward(false).forward(false);
        assert_eq!(b.carrets[0].index, 6);
    }
    // #[test]
    // fn rope_forward() {
    //     let indexes = vec![1usize, 2, 3, 4, 5, 7, 8, 9, 10, 11, 12, 12];
    //     let mut b = Buffer::new().insert("hello\r\nWorld");
    //     b.carrets[0].index = 0;

    //     for i in &indexes {
    //         b = b.forward(false);
    //         assert_eq!(b.carrets[0].index, *i);
    //     }
    //     b.carrets[0].index = 0;

    //     for i in &indexes {
    //         b = b.forward(true);
    //         assert_eq!(b.carrets[0].selection.byte_len, *i);
    //         assert_eq!(
    //             b.carrets[0].selection.direction,
    //             SelectionDirection::Backward
    //         );
    //     }
    // }
    // #[test]
    // fn rope_backward() {
    //     let indexes = vec![11, 10, 9, 8, 7, 5, 4, 3, 2, 1, 0, 0];
    //     let mut b = Buffer::new().insert("hello\r\nWorld");

    //     for i in &indexes {
    //         b = b.backward(false);
    //         assert_eq!(b.carrets[0].index, *i);
    //     }
    //     let mut b = Buffer::new().insert("hello\r\nWorld");
    //     let len = vec![1, 2, 3, 4, 5, 7, 8, 9, 10, 11, 12, 12];
    //     for i in &len {
    //         b = b.backward(true);
    //         assert_eq!(b.carrets[0].selection.byte_len, *i);
    //         assert_eq!(
    //             b.carrets[0].selection.direction,
    //             SelectionDirection::Forward
    //         );
    //     }
    // }
}
