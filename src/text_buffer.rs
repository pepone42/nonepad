use crate::carret::Carrets;
use std::cell::RefCell;
use std::io::Result;
use std::ops::{AddAssign, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo};
use std::path::Path;
use std::rc::Rc;

use ropey::{Rope, RopeSlice};
use unicode_segmentation::{GraphemeCursor, GraphemeIncomplete};

use crate::carret::Carret;
use crate::file::{Indentation, TextFileInfo};
use crate::rope_utils::*;

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
        let buffer = Buffer::from_rope(file.1);
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

    pub fn selection_on_line<'a>(&'a self, line_idx: usize, ranges: &mut Vec<SelectionLineRange>) {
        ranges.clear();
        for r in self.buffer.carrets.iter().filter_map(move |c| {
            if !c.selection_is_empty() {
                let r = c.range();
                match (
                    self.buffer.rope.byte_to_line(r.start.0),
                    self.buffer.rope.byte_to_line(r.end.0),
                ) {
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
        index - self.buffer.rope.line_to_byte(self.buffer.rope.byte_to_line(index))
    }

    pub fn byte_to_line(&self, index: usize) -> usize {
        self.buffer.rope.byte_to_line(index)
    }

    pub fn byte_to_line_range(&self, range: Range<AbsoluteIndex>) -> RangeInclusive<usize> {
        let line_start = self.buffer.rope.byte_to_line(range.start.0);
        let line_end = self.buffer.rope.byte_to_line(range.end.0);
        if self.byte_to_line_relative_index(range.end.0) == 0 && line_start != line_end {
            line_start..=line_end - 1
        } else {
            line_start..=line_end
        }
    }

    pub fn line(&self, line: usize) -> Line {
        Line::new(line)
    }

    /// Construct a string with tab replaced as space
    pub fn displayable_line(&self, line: usize, out: &mut String, indices: &mut Vec<RelativeIndex>) {
        self.line(line).get_displayable_string(&self.buffer.rope,self.file.indentation.visible_len(), out, indices);
    }

    pub fn carrets_on_line<'a>(&'a self, line_idx: usize) -> impl Iterator<Item = &'a Carret> {
        self.buffer.carrets.iter().filter(move |c| c.line == line_idx)
    }
    
    pub fn backward(&mut self, expand_selection: bool) {
        let mut buf = self.buffer.clone();
        for s in &mut buf.carrets.iter_mut() {
            s.move_backward(expand_selection,&self.buffer.rope, self.file.indentation.visible_len());
        }

        buf.carrets.merge();
        self.buffer = buf;
    }

    pub fn forward(&mut self, expand_selection: bool) {
        let mut buf = self.buffer.clone();
        for s in &mut buf.carrets.iter_mut() {
            s.move_forward(expand_selection,&buf.rope,self.file.indentation.visible_len());
        }

        buf.carrets.merge();
        self.buffer=buf;
    }

    pub fn up(&mut self, expand_selection: bool) {
        let mut buf = self.buffer.clone();
        for s in &mut buf.carrets.iter_mut() {
            s.move_up(expand_selection,&buf.rope,self.file.indentation.visible_len());
        }

        buf.carrets.merge();
        self.buffer = buf;
    }
    pub fn down(&mut self, expand_selection: bool) {
        let mut buf = self.buffer.clone();
        for s in &mut buf.carrets.iter_mut() {
            s.move_down(expand_selection,&buf.rope,self.file.indentation.visible_len());
        }

        buf.carrets.merge();
        self.buffer = buf;
    }
    pub fn duplicate_down(&mut self) {
        let tabsize = self.file.indentation.visible_len();
        let mut buf = self.buffer.clone();
        buf.carrets.sort_unstable();

        if let Some(c) = buf.carrets.last().and_then(|c| c.duplicate_down(&buf.rope,tabsize)) {
            buf.carrets.push(c);
        }
        buf.carrets.merge();
        self.buffer = buf;
    }

    pub fn duplicate_up(&mut self) {
        let tabsize = self.file.indentation.visible_len();
        let mut buf = self.buffer.clone();
        buf.carrets.sort_unstable();

        if let Some(c) = buf.carrets.first().and_then(|c| c.duplicate_up(&buf.rope,tabsize)) {
            buf.carrets.push(c);
        }
        buf.carrets.merge();
        self.buffer = buf;
    }

    pub fn cancel_selection(&mut self) {
        let mut buf = self.buffer.clone();
        for c in &mut buf.carrets.iter_mut() {
            c.cancel_selection();
        }
        self.buffer = buf
    }

    pub fn have_selection(&self) -> bool {
        self.buffer.carrets.iter().any(|c| !c.selection_is_empty())
    }

    pub fn revert_to_single_carrets(&mut self) {
        let mut buf = self.buffer.clone();
        buf.carrets.retain(|c| !c.is_clone);
        buf.carrets.merge();
        self.buffer = buf;
    }

    pub fn end(&mut self, expand_selection: bool) {
        let tabsize = self.file.indentation.visible_len();
        let mut buf = self.buffer.clone();
        for s in &mut buf.carrets.iter_mut() {
            s.move_end(expand_selection,&buf.rope,tabsize);
        }

        buf.carrets.merge();
        self.buffer = buf;
    }

    pub fn home(&mut self, expand_selection: bool) {
        let mut buf = self.buffer.clone();
        let tabsize = self.file.indentation.visible_len();
        for s in &mut buf.carrets.iter_mut() {
            s.move_home(expand_selection,&buf.rope,tabsize);
        }

        buf.carrets.merge();
        self.buffer = buf
    }

    pub fn insert(&mut self, text: &str) {
        let mut buf = self.buffer.clone();
        let tabsize = self.file.indentation.visible_len();

        for i in 0..buf.carrets.len() {
            let r= buf.carrets[i].range();
            buf.edit(&r,text,tabsize);
            buf.carrets[i].set_index(r.start + text.len(), true,&buf.rope,tabsize);
        }
        buf.carrets.merge();

        self.push_edit(buf);
    }

    pub fn backspace(&mut self) {
        let mut buf = self.buffer.clone();
        let tabsize = self.file.indentation.visible_len();

        let mut did_nothing = true;
        for i in 0..buf.carrets.len() {
            if !buf.carrets[i].selection_is_empty() {
                // delete all the selection
                let r= buf.carrets[i].range();
                buf.edit(&r,"",tabsize);
                buf.carrets[i].set_index(r.start, true,&buf.rope,tabsize);

                did_nothing = false;
            } else if buf.carrets[i].index.0 > 0 {
                // delete the preceding grapheme
                let r = AbsoluteIndex(prev_grapheme_boundary(&buf.rope.slice(..), buf.carrets[i].index.0))
                    ..buf.carrets[i].index;
                buf.edit(&r,"",tabsize);
                buf.carrets[i].set_index(r.start, true,&buf.rope,tabsize);

                did_nothing = false;
            } else {
                continue;
            }
        }
        if !did_nothing {
            buf.carrets.merge();
            self.push_edit(buf);
        }
    }

    pub fn delete(&mut self) {
        let mut buf = self.buffer.clone();
        let tabsize = self.file.indentation.visible_len();
        let mut did_nothing = true;
        for i in 0..buf.carrets.len() {
            if !buf.carrets[i].selection_is_empty() {
                let r = buf.carrets[i].range();
                buf.edit(&r,"",tabsize);
                buf.carrets[i].set_index(r.start, true,&buf.rope,tabsize);

                did_nothing = false;
            } else if buf.carrets[i].index.0 < buf.rope.len_bytes() {
                let r = buf.carrets[i].index
                    ..AbsoluteIndex(next_grapheme_boundary(&buf.rope.slice(..), buf.carrets[i].index.0));
                buf.edit(&r,"",tabsize);
                buf.carrets[i].set_index(r.start, true,&buf.rope,tabsize);

                did_nothing = false;
            } else {
                continue;
            }
        }
        if !did_nothing {
            buf.carrets.merge();
            self.push_edit(buf);
        }
    }

    pub fn tab(&mut self) {
        let mut buf = self.buffer.clone();
        let tabsize = self.file.indentation.visible_len();

        for i in 0..buf.carrets.len() {
            if let Some(line_range) = buf.carrets[i].selected_lines_range(&buf.rope) {
                for line in line_range {
                    let inserted_byte: usize;
                    let line_char = buf.rope.line_to_char(line);
                    let line_byte = buf.rope.line_to_byte(line);
                    match self.file.indentation {
                        Indentation::Space(n) => {
                            let start = line_indent_len(&buf.rope.slice(..), line, n);
                            let nb_space = n - start % n;
                            buf.rope.insert(line_char, &" ".repeat(nb_space));
                            inserted_byte = nb_space;
                        }
                        Indentation::Tab(_) => {
                            buf.rope.insert_char(line_char, '\t');
                            inserted_byte = 1;
                        }
                    }
                    for j in i..buf.carrets.len() {
                        buf.carrets[j].update_after_insert(AbsoluteIndex(line_byte), inserted_byte,&buf.rope,tabsize);
                    }
                }
            } else {
                let r = buf.carrets[i].range();
                let text = match self.file.indentation {
                    Indentation::Space(n) => {
                        // let i = r.start - rope.line_to_byte(*line_range.start());
                        // let start = line_index_to_column(&rope.line(*line_range.start()),i,n);
                        let start = buf.carrets[i].column_index(&buf.rope,tabsize);
                        let nb_space = n - start.0 % n;
                        " ".repeat(nb_space).to_owned()
                    }
                    Indentation::Tab(_) => "\t".to_owned(),
                };

                //carrets[i].index = r.start;
                let cr_start = buf.rope.byte_to_char(r.start.0);
                let cr_end = buf.rope.byte_to_char(r.end.0);
                buf.rope.remove(cr_start..cr_end);
                buf.rope.insert(cr_start, &text);

                //carrets[i].selection = Default::default();
                buf.carrets[i].set_index(r.start + text.len(), true,&buf.rope,tabsize); // assume text have the correct grapheme boundary
                for j in i + 1..buf.carrets.len() {
                    buf.carrets[j].update_after_delete(r.start, r.end.0 - r.start.0,&buf.rope,tabsize); // TODO verify this
                    buf.carrets[j].update_after_insert(r.start, text.len(),&buf.rope,tabsize);
                    // if let Some(ref mut sel) = buf.carrets[j].selection {
                    //     *sel -= r.end - r.start;
                    //     *sel += text.len();
                    // }
                }
            }
        }
        buf.carrets.merge();
        self.push_edit(buf);
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
    pub carrets: Carrets,
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            rope: Rope::new(),
            carrets: Carrets::new(),
        }
    }


    pub fn from_rope(rope: Rope) -> Self {
        Self {
            rope: rope.clone(),
            carrets: Carrets::new(),
        }
    }

    pub fn edit(&mut self, range: &Range<AbsoluteIndex>, text: &str, tabsize: usize) {
        let insert_index = self.rope.byte_to_char(range.start.0);
        let end_index = self.rope.byte_to_char(range.end.0);
        let cr = insert_index..end_index;
        self.rope.remove(cr);
        self.rope.insert(insert_index, text);

        for i in 0..self.carrets.len() {
            self.carrets[i].update_after_delete(range.start, range.end.0 - range.start.0, &self.rope,tabsize); // TODO verify this
            self.carrets[i].update_after_insert(range.start, text.len(),&self.rope, tabsize);
        }
        self.carrets.merge();
    }

}

impl ToString for Buffer {
    fn to_string(&self) -> String {
        self.rope.to_string()
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
