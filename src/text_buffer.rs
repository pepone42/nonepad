use std::io::{Read, Result};
use std::ops::{Bound, Range, RangeBounds};

use ropey::{str_utils::byte_to_char_idx, Rope, RopeSlice};
use unicode_segmentation::{GraphemeCursor, GraphemeIncomplete};

use crate::file::TextFile;

/// Finds the previous grapheme boundary before the given char position.
fn prev_grapheme_boundary(slice: &RopeSlice, byte_idx: usize) -> usize {
    // Bounds check
    debug_assert!(byte_idx <= slice.len_bytes());

    // Get the chunk with our byte index in it.
    let (mut chunk, mut chunk_byte_idx, _, _) = slice.chunk_at_byte(byte_idx);

    // Set up the grapheme cursor.
    let mut gc = GraphemeCursor::new(byte_idx, slice.len_bytes(), true);

    // Find the previous grapheme cluster boundary.
    loop {
        match gc.prev_boundary(chunk, chunk_byte_idx) {
            Ok(None) => return 0,
            Ok(Some(n)) => {
                let tmp = n - chunk_byte_idx;
                return chunk_byte_idx + tmp;
            }
            Err(GraphemeIncomplete::PrevChunk) => {
                let (a, b, _, _) = slice.chunk_at_byte(chunk_byte_idx - 1);
                chunk = a;
                chunk_byte_idx = b;
            }
            Err(GraphemeIncomplete::PreContext(n)) => {
                let ctx_chunk = slice.chunk_at_byte(n - 1).0;
                gc.provide_context(ctx_chunk, n - ctx_chunk.len());
            }
            _ => unreachable!(),
        }
    }
}

/// Finds the next grapheme boundary after the given char position.
fn next_grapheme_boundary(slice: &RopeSlice, byte_idx: usize) -> usize {
    // Bounds check
    debug_assert!(byte_idx <= slice.len_bytes());

    // Get the chunk with our byte index in it.
    let (mut chunk, mut chunk_byte_idx, _, _) = slice.chunk_at_byte(byte_idx);

    // Set up the grapheme cursor.
    let mut gc = GraphemeCursor::new(byte_idx, slice.len_bytes(), true);

    // Find the next grapheme cluster boundary.
    loop {
        match gc.next_boundary(chunk, chunk_byte_idx) {
            Ok(None) => return slice.len_chars(),
            Ok(Some(n)) => {
                let tmp = n - chunk_byte_idx;
                return chunk_byte_idx + tmp;
            }
            Err(GraphemeIncomplete::NextChunk) => {
                chunk_byte_idx += chunk.len();
                let (a, b, _, _) = slice.chunk_at_byte(chunk_byte_idx);
                chunk = a;
                chunk_byte_idx = b;
            }
            Err(GraphemeIncomplete::PreContext(n)) => {
                let ctx_chunk = slice.chunk_at_byte(n - 1).0;
                gc.provide_context(ctx_chunk, n - ctx_chunk.len());
            }
            _ => unreachable!(),
        }
    }
}

fn index_to_point(slice: &RopeSlice, index: usize) -> (usize, usize) {
    let line = slice.byte_to_line(index);
    let line_index = slice.line_to_byte(line);
    let mut i = next_grapheme_boundary(slice, line_index);
    let mut col = 0;
    while i < index {
        i = next_grapheme_boundary(slice, i);
        col += 1;
    }
    (col, line)
}

#[derive(Debug, Clone, Copy)]
pub struct Carret {
    pub index: usize,
    vcol: usize,
    pub col_index: usize,
    selection: Selection,
}

impl Carret {
    pub fn new() -> Self {
        Self {
            index: 0,
            vcol: 0,
            col_index: 0,
            selection: Default::default(),
        }
    }

    pub fn range(&self, selection: Selection) -> Range<usize> {
        match selection.direction {
            SelectionDirection::Forward => self.index..self.index + selection.len,
            SelectionDirection::Backward => self.index - selection.len..self.index,
        }
    }
}

#[derive(Debug, Default)]
pub struct EditStack {
    stack: Vec<Buffer>,
    sp: usize,
    pub file: TextFile,
}


impl EditStack {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from_file(file: TextFile) -> Self {
        let b = Buffer::from_rope(file.buffer.clone());
        Self {
            stack: vec![b],
            sp: 1,
            file,
        }
    }

    pub fn buffer(&self) -> Option<&Buffer> {
        if self.sp > 0 {
            Some(&self.stack[self.sp - 1])
        } else {
            None
        }
    }

    pub fn buffer_mut(&mut self) -> Option<&mut Buffer> {
        if self.sp > 0 {
            Some(&mut self.stack[self.sp - 1])
        } else {
            None
        }
    }

    // pub fn line(&self, line: usize, out: &mut String) {
    //     out.clear();
    //     if self.sp > 0 {
    //         self.stack[self.sp - 1].line(line, out);
    //     }
    // }

    pub fn push(&mut self, buffer: Buffer) {
        self.stack.truncate(self.sp);
        self.stack.push(buffer);
        self.sp += 1;
    }

    pub fn peek(&self) -> Option<Buffer> {
        if self.sp == 0 {
            None
        } else {
            Some(self.stack[self.sp - 1].clone())
        }
    }

    pub fn undo(&mut self) -> Option<Buffer> {
        if self.sp <=1 {
            None
        } else {
            self.sp -= 1;
            Some(self.stack[self.sp].clone())
        }
    }

    pub fn redo(&mut self) -> Option<Buffer> {
        if self.sp == self.stack.len() {
            None
        } else {
            let result = self.stack[self.sp - 1].clone();
            self.sp += 1;
            Some(result)
        }
    }

    pub fn forward(&mut self, expand_selection: bool) {
        if self.sp > 0 {
            self.stack[self.sp - 1] = self.stack[self.sp - 1].forward(expand_selection);
        }
    }
    pub fn backward(&mut self, expand_selection: bool) {
        if self.sp > 0 {
            self.stack[self.sp - 1] = self.stack[self.sp - 1].backward(expand_selection);
        }
    }
    pub fn insert(&mut self, text: &str) {
        let topbuf = self.peek().unwrap_or_else(|| Buffer::new());
        self.push(topbuf.insert(text));
    }
    pub fn backspace(&mut self) {
        let topbuf = self.peek().unwrap_or_else(|| Buffer::new());
        if let Some(b) = topbuf.backspace() {self.push(b)};
    }
    pub fn delete(&mut self) {
        let topbuf = self.peek().unwrap_or_else(|| Buffer::new());
        if let Some(b) = topbuf.delete() {self.push(b)};
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionDirection {
    Forward,
    Backward,
}

#[derive(Debug, Clone, Copy)]
pub struct Selection {
    direction: SelectionDirection,
    pub len: usize,
}

impl Selection {
    pub fn new() -> Self {
        Selection {
            direction: SelectionDirection::Forward,
            len: 0,
        }
    }
}

impl Default for Selection {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct Buffer {
    pub rope: Rope,
    pub carrets: Vec<Carret>,
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            rope: Rope::new(),
            carrets: {
                let mut v = Vec::new();
                v.push(Carret::new());
                v
            },
        }
    }

    pub fn carrets_on_line<'a>(&'a self, line_idx: usize) -> impl Iterator<Item = &'a Carret> {
        self.carrets.iter().filter(move |c| self.rope.byte_to_line(c.index) == line_idx)
    }

    pub fn from_rope(rope: Rope) -> Self {
        Self {
            rope,
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

    pub fn backward(&self, expand_selection: bool) -> Self {
        let rope = self.rope.clone();
        let mut carrets = self.carrets.clone();
        for s in &mut carrets {
            let index = prev_grapheme_boundary(&rope.slice(..), s.index);

            if expand_selection && s.index != index {
                s.selection.direction = SelectionDirection::Forward;
                s.selection.len += s.index - index;
            }
            s.index = index;
            let (vcol, line) = index_to_point(&rope.slice(..), s.index);
            s.vcol = vcol;
            s.col_index = s.index - rope.line_to_byte(line);
        }
        Self { rope, carrets }
    }

    pub fn forward(&self, expand_selection: bool) -> Self {
        let rope = self.rope.clone();
        let mut carrets = self.carrets.clone();
        for s in &mut carrets {
            let index = next_grapheme_boundary(&rope.slice(..), s.index);

            if expand_selection && s.index != index {
                s.selection.direction = SelectionDirection::Backward;
                s.selection.len += index - s.index;
            }
            s.index = index;
            let (vcol, line) = index_to_point(&rope.slice(..), s.index);
            s.vcol = vcol;
            s.col_index = s.index - rope.line_to_byte(line);
        }
        Self { rope, carrets }
    }

    pub fn insert(&self, text: &str) -> Self {
        let mut rope = self.rope.clone();
        let mut carrets = self.carrets.clone();
        for s in &mut carrets {
            let r = s.range(s.selection);
            s.index = r.start;
            rope.remove(rope.byte_to_char(r.start)..rope.byte_to_char(r.end));
            rope.insert(rope.byte_to_char(s.index), text);

            s.index += text.len(); // assume text have the correct grapheme boundary
            s.selection = Default::default();
            let (vcol, line) = index_to_point(&rope.slice(..), s.index);
            s.vcol = vcol;
            s.col_index = s.index - rope.line_to_byte(line);
        }
        Self { rope, carrets }
    }

    pub fn backspace(&self) -> Option<Self> {
        let mut rope = self.rope.clone();
        let mut carrets = self.carrets.clone();
        let mut did_nothing = true;
        for s in &mut carrets {
            if s.selection.len > 0 {
                let r = s.range(s.selection);
                s.index = r.start;
                rope.remove(rope.byte_to_char(r.start)..rope.byte_to_char(r.end));
                s.selection = Default::default();
                did_nothing=false;
            } else if s.index > 0 {
                let r = prev_grapheme_boundary(&rope.slice(..), s.index)..s.index;
                s.index = r.start;
                rope.remove(rope.byte_to_char(r.start)..rope.byte_to_char(r.end));
                did_nothing=false;
            } else {
                continue;
            }
            let (vcol, line) = index_to_point(&rope.slice(..), s.index);
            s.vcol = vcol;
            s.col_index = s.index - rope.line_to_byte(line);
        }
        if did_nothing {
            None
        } else {
            Some(Self { rope, carrets })
        }
    }
    
    pub  fn delete(&self) -> Option<Self> {
        let mut rope = self.rope.clone();
        let mut carrets = self.carrets.clone();
        let mut did_nothing = true;
        for s in &mut carrets {
            if s.selection.len > 0 {
                let r = s.range(s.selection);
                s.index = r.start;
                rope.remove(rope.byte_to_char(r.start)..rope.byte_to_char(r.end));
                s.selection = Default::default();
                did_nothing=false;
            } else if s.index < rope.len_bytes()-1 {
                let r = s.index..next_grapheme_boundary(&rope.slice(..), s.index);
                s.index = r.start;
                rope.remove(rope.byte_to_char(r.start)..rope.byte_to_char(r.end));
                did_nothing=false;
            } else {
                continue;
            }
            let (vcol, line) = index_to_point(&rope.slice(..), s.index);
            s.vcol = vcol;
            s.col_index = s.index - rope.line_to_byte(line);
        }
        if did_nothing {
            None
        } else {
            Some(Self { rope, carrets })
        }
    }

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
        assert_eq!(
            b.insert("hello").insert(" world").to_string(),
            "hello world"
        );
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
    #[test]
    fn rope_forward() {
        let indexes = vec![1usize, 2, 3, 4, 5, 7, 8, 9, 10, 11, 12, 12];
        let mut b = Buffer::new().insert("hello\r\nWorld");
        b.carrets[0].index = 0;

        for i in &indexes {
            b = b.forward(false);
            assert_eq!(b.carrets[0].index, *i);
        }
        b.carrets[0].index = 0;

        for i in &indexes {
            b = b.forward(true);
            assert_eq!(b.carrets[0].selection.len, *i);
            assert_eq!(
                b.carrets[0].selection.direction,
                SelectionDirection::Backward
            );
        }
    }
    #[test]
    fn rope_backward() {
        let indexes = vec![11, 10, 9, 8, 7, 5, 4, 3, 2, 1, 0, 0];
        let mut b = Buffer::new().insert("hello\r\nWorld");

        for i in &indexes {
            b = b.backward(false);
            assert_eq!(b.carrets[0].index, *i);
        }
        let mut b = Buffer::new().insert("hello\r\nWorld");
        let len = vec![1, 2, 3, 4, 5, 7, 8, 9, 10, 11, 12, 12];
        for i in &len {
            b = b.backward(true);
            assert_eq!(b.carrets[0].selection.len, *i);
            assert_eq!(
                b.carrets[0].selection.direction,
                SelectionDirection::Forward
            );
        }
    }
}
