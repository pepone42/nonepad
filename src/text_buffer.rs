use std::io::Result;
use std::ops::{Range, RangeFrom, RangeFull, RangeTo};
use std::path::Path;

use ropey::{Rope, RopeSlice};
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
    line_boundary(slice, line)
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

fn byte_to_line_first_column(slice: &RopeSlice, index: usize) -> usize {
    let range = byte_to_line_boundary(slice, index);
    let mut start = range.start;
    let char_range = slice.byte_to_char(range.start)..slice.byte_to_char(range.end);
    for c in slice.slice(char_range).chars() {
        if c != '\t' && c != ' ' {
            break;
        }
        start += c.len_utf8();
    }
    if index == range.start {
        start
    } else if start >= index {
        range.start
    } else {
        start
    }
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
    let line_boundary = line_boundary(slice, line);
    let mut index = line_boundary.start;

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

fn collapse_selections(carrets: &mut Vec<Carret>) {
    if carrets.len() > 1 {
        carrets.sort_unstable_by(|a, b| a.range().start.cmp(&b.range().start))
    }
    let mut redo = true;
    'outer: while redo {
        for i in 0..carrets.len() - 1 {
            if carrets[i].range().contains(&carrets[i + 1].range().start)
                || (carrets[i].selection.is_none() && carrets[i].index == carrets[i + 1].index)
            {
                carrets[i] = Carret::merge(&carrets[i], &carrets[i + 1]);
                carrets.remove(i + 1);
                redo = true;
                continue 'outer;
            }
        }
        redo = false;
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
    pub byte_len: usize,
}

impl Selection {
    pub fn new() -> Self {
        Selection {
            direction: SelectionDirection::Forward,
            byte_len: 0,
        }
    }
}

impl Default for Selection {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Carret {
    pub index: usize,
    vcol: usize,
    pub col_index: usize,
    pub selection: Option<usize>,
    is_clone: bool,
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

    // TODO: false
    pub fn merge(c1: &Carret, c2: &Carret) -> Self {
        let (cstart, cend) = if c1.range().start < c2.range().start {
            (c1, c2)
        } else {
            (c2, c1)
        };
        Self {
            index: cstart.index,
            vcol: cstart.vcol,
            col_index: cstart.col_index,
            selection: Some(cend.range().end),
            is_clone: cstart.is_clone || cend.is_clone,
        }
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
    pub fn char_range(&self, slice: &RopeSlice) -> Range<usize> {
        let r = self.range();
        slice.byte_to_char(r.start)..slice.byte_to_char(r.end)
    }

    pub fn selection_grapheme_len(&self, slice: &RopeSlice) -> usize {
        let r = self.range();
        let mut index = r.start;
        let mut i = 0;
        while index < r.end {
            index = next_grapheme_boundary(slice, index);
            i += 1;
        }
        return i;
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
    pub fn duplicate_down(&mut self) {
        self.buffer = self.buffer.duplicate_down();
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
        Self {
            rope: Rope::new(),
            carrets: {
                let mut v = Vec::new();
                v.push(Carret::default());
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

    pub fn from_rope(rope: Rope) -> Self {
        Self {
            rope,
            carrets: {
                let mut v = Vec::new();
                v.push(Carret::default());
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

            if expand_selection {
                if s.selection.is_none() {
                    s.selection = Some(s.index);
                }
            } else {
                s.selection = None;
            };
            s.index = index;
            let (vcol, line) = index_to_point(&rope.slice(..), s.index);
            s.vcol = vcol;
            s.col_index = s.index - rope.line_to_byte(line);
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
            s.index = index;
            let (vcol, line) = index_to_point(&rope.slice(..), s.index);
            s.vcol = vcol;
            s.col_index = s.index - rope.line_to_byte(line);
        }
        collapse_selections(&mut carrets);
        Self { rope, carrets }
    }

    pub fn up(&self, expand_selection: bool) -> Self {
        let rope = self.rope.clone();
        let mut carrets = self.carrets.clone();
        for s in &mut carrets {
            let line = rope.byte_to_line(s.index);
            if line > 0 {
                let (index, col_index, _) = point_to_index(&rope.slice(..), s.vcol, line - 1);

                if expand_selection {
                    if s.selection.is_none() {
                        s.selection = Some(s.index);
                    }
                } else {
                    s.selection = None;
                };
                s.col_index = col_index;
                s.index = index;
            }
        }
        collapse_selections(&mut carrets);
        Self { rope, carrets }
    }
    pub fn down(&self, expand_selection: bool) -> Self {
        let rope = self.rope.clone();
        let mut carrets = self.carrets.clone();
        for s in &mut carrets {
            let line = rope.byte_to_line(s.index);
            if line < self.rope.len_lines() - 1 {
                let (index, col_index, _) = point_to_index(&rope.slice(..), s.vcol, line + 1);

                if expand_selection {
                    if s.selection.is_none() {
                        s.selection = Some(s.index);
                    }
                } else {
                    s.selection = None;
                };

                s.col_index = col_index;
                s.index = index;
            }
        }
        collapse_selections(&mut carrets);
        Self { rope, carrets }
    }
    pub fn duplicate_down(&self) -> Self {
        let rope = self.rope.clone();
        let mut carrets = self.carrets.clone();
        carrets.sort_unstable_by(|a, b| a.range().cmp(b.range()));
        let mut c = carrets.last().unwrap().clone();

        let line = rope.byte_to_line(c.index);
        if line < self.rope.len_lines() - 1 {
            let (index, col_index, _) = point_to_index(&rope.slice(..), c.vcol, line + 1);

            c.col_index = col_index;
            c.index = index;
            c.is_clone = true;
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
            s.index = index;
            let (vcol, _) = index_to_point(&rope.slice(..), s.index);
            s.vcol = vcol;
            s.col_index = s.index - line_boundary.start;
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
            s.index = index;
            let (vcol, _) = index_to_point(&rope.slice(..), s.index);
            s.vcol = vcol;
            s.col_index = s.index - line_boundary.start;
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

            carrets[i].index += text.len(); // assume text have the correct grapheme boundary

            for j in i + 1..carrets.len() {
                carrets[j].index -= r.end - r.start; // TODO verify this
                carrets[j].index += text.len();
                if let Some(ref mut sel) = carrets[j].selection {
                    *sel -= r.end - r.start;
                    *sel += text.len();
                }
            }

            carrets[i].selection = Default::default();
            let (vcol, line) = index_to_point(&rope.slice(..), carrets[i].index);
            carrets[i].vcol = vcol;
            carrets[i].col_index = carrets[i].index - rope.line_to_byte(line);
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
                carrets[i].index = r.start;
                rope.remove(rope.byte_to_char(r.start)..rope.byte_to_char(r.end));

                // update all others cursors
                for j in i + 1..carrets.len() {
                    carrets[j].index -= r.end - r.start; // TODO verify this
                    if let Some(ref mut sel) = carrets[j].selection {
                        *sel -= r.end - r.start;
                    }
                }
                carrets[i].selection = Default::default();
                did_nothing = false;
            } else if carrets[i].index > 0 {
                // delete the preceding grapheme
                let r = prev_grapheme_boundary(&rope.slice(..), carrets[i].index)..carrets[i].index;
                carrets[i].index = r.start;
                rope.remove(rope.byte_to_char(r.start)..rope.byte_to_char(r.end));

                // update all others cursors
                for j in i + 1..carrets.len() {
                    carrets[j].index -= r.end - r.start;
                }
                did_nothing = false;
            } else {
                continue;
            }
            let (vcol, line) = index_to_point(&rope.slice(..), carrets[i].index);
            carrets[i].vcol = vcol;
            carrets[i].col_index = carrets[i].index - rope.line_to_byte(line);
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
                carrets[i].index = r.start;
                rope.remove(rope.byte_to_char(r.start)..rope.byte_to_char(r.end));

                // update all others cursors
                for j in i + 1..carrets.len() {
                    carrets[j].index -= r.end - r.start; // TODO verify this
                    if let Some(ref mut sel) = carrets[j].selection {
                        *sel -= r.end - r.start;
                    }
                }
                carrets[i].selection = Default::default();
                did_nothing = false;
            } else if carrets[i].index < rope.len_bytes() - 1 {
                let r = carrets[i].index..next_grapheme_boundary(&rope.slice(..), carrets[i].index);
                carrets[i].index = r.start;
                rope.remove(rope.byte_to_char(r.start)..rope.byte_to_char(r.end));
                // update all others cursors
                for j in i + 1..carrets.len() {
                    carrets[j].index -= r.end - r.start;
                }
                did_nothing = false;
            } else {
                continue;
            }
            let (vcol, line) = index_to_point(&rope.slice(..), carrets[i].index);
            carrets[i].vcol = vcol;
            carrets[i].col_index = carrets[i].index - rope.line_to_byte(line);
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
