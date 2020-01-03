use std::io::{Read, Result};
use std::ops::{Bound, Range, RangeBounds};
use std::path::Path;

use ropey::{str_utils::byte_to_char_idx, Rope, RopeSlice};
use unicode_segmentation::{GraphemeCursor, GraphemeIncomplete};

use crate::file::TextFileInfo;

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
            Ok(None) => return slice.len_bytes(),
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

fn byte_to_line_boundary(slice: &RopeSlice, index: usize) -> Range<usize> {
    let line = slice.byte_to_line(index);
    line_boundary(slice,line)
}

fn line_boundary(slice: &RopeSlice, line: usize) -> Range<usize> {
    let line_start = slice.line_to_byte(line);
    let line_end = if line + 1 >= slice.len_lines() {
        slice.len_bytes()
    } else {
        prev_grapheme_boundary(slice, slice.line_to_byte(line + 1))
    };
    line_start..line_end
}

fn index_to_point(slice: &RopeSlice, index: usize) -> (usize, usize) {
    let line = slice.byte_to_line(index);
    let line_index = slice.line_to_byte(line);
    let mut i = line_index;
    let mut col = 0;
    while i < index {
        i = next_grapheme_boundary(slice, i);
        col += 1;
    }
    (col, line)
}

fn point_to_index(slice: &RopeSlice, vcol: usize, line: usize) -> (usize, usize, usize) {
    let line_boundary = line_boundary(slice,line);
    let mut index= line_boundary.start;

    let mut col = 0;
    for _ in 0..vcol {
        if index >= line_boundary.end {
            break;
        }
        col += 1;
        index = next_grapheme_boundary(slice, index);
    }
    (index, index - line_boundary.start, col)
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

#[derive(Debug, Clone, Default)]
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
        self.carrets
            .iter()
            .filter(move |c| self.rope.byte_to_line(c.index) == line_idx)
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

    pub fn up(&self, expand_selection: bool) -> Self {
        let rope = self.rope.clone();
        let mut carrets = self.carrets.clone();
        for s in &mut carrets {
            let line = rope.byte_to_line(s.index);
            if line > 0 {
                let (index, col_index, _) = point_to_index(&rope.slice(..), s.vcol, line - 1);

                s.col_index = col_index;
                s.index = index;
            }
        }
        Self { rope, carrets }
    }
    pub fn down(&self, expand_selection: bool) -> Self {
        let rope = self.rope.clone();
        let mut carrets = self.carrets.clone();
        for s in &mut carrets {
            let line = rope.byte_to_line(s.index);
            if line < self.rope.len_lines() - 1 {
                let (index, col_index, _) = point_to_index(&rope.slice(..), s.vcol, line + 1);

                s.col_index = col_index;
                s.index = index;
            }
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
                did_nothing = false;
            } else if s.index > 0 {
                let r = prev_grapheme_boundary(&rope.slice(..), s.index)..s.index;
                s.index = r.start;
                rope.remove(rope.byte_to_char(r.start)..rope.byte_to_char(r.end));
                did_nothing = false;
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

    pub fn delete(&self) -> Option<Self> {
        let mut rope = self.rope.clone();
        let mut carrets = self.carrets.clone();
        let mut did_nothing = true;
        for s in &mut carrets {
            if s.selection.len > 0 {
                let r = s.range(s.selection);
                s.index = r.start;
                rope.remove(rope.byte_to_char(r.start)..rope.byte_to_char(r.end));
                s.selection = Default::default();
                did_nothing = false;
            } else if s.index < rope.len_bytes() - 1 {
                let r = s.index..next_grapheme_boundary(&rope.slice(..), s.index);
                s.index = r.start;
                rope.remove(rope.byte_to_char(r.start)..rope.byte_to_char(r.end));
                did_nothing = false;
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
