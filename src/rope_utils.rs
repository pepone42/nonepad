use ropey::Rope;
use std::ops::{Range,Add,Sub,AddAssign,SubAssign};
use std::cell::RefCell;
use std::rc::Rc;
use ropey::RopeSlice;
use unicode_segmentation::{GraphemeCursor, GraphemeIncomplete};
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct AbsoluteIndex(pub usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct RelativeIndex(pub usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Column(pub usize);

impl Add<usize> for AbsoluteIndex {
    type Output=AbsoluteIndex;
    fn add(self, rhs: usize) -> Self::Output { 
        AbsoluteIndex(self.0 + rhs)
    }
}
impl Sub<usize> for AbsoluteIndex {
    type Output=AbsoluteIndex;
    fn sub(self, rhs: usize) -> Self::Output { 
        AbsoluteIndex(self.0 - rhs)
    }
}
impl AddAssign<usize> for AbsoluteIndex {
    fn add_assign(&mut self, rhs: usize) { 
        self.0 += rhs
    }
}
impl SubAssign<usize> for AbsoluteIndex {
    fn sub_assign(&mut self, rhs: usize) { 
        self.0 -= rhs
    }
}

pub struct Line {
    owner: Rc<RefCell<Rope>>,
    pub line: usize,
}

impl Line {
    pub fn new(line: usize, owner: Rc<RefCell<Rope>>) -> Self {
        Self { owner, line }
    }
    pub fn for_index(index: AbsoluteIndex, owner: Rc<RefCell<Rope>>) -> Self {
        let line = owner.borrow().byte_to_line(index.0);
        Self {
            owner,
            line,
        }
    }

    pub fn rope(&self) -> Rope {
        //self.owner.line(self.line)
        let boundary = self.boundary();
        let r = self.owner.borrow();
        r.slice(boundary.start.0..boundary.end.0).into()
    }
    pub fn get_string(&self, out: &mut String) {
        out.clear();
        // todo
    }

    pub fn prev_line(&self) -> Option<Self> {
        if self.line == 0 {
            None
        } else {
            Some(Self {
                owner: self.owner.clone(),
                line: self.line - 1,
            })
        }
    }

    pub fn next_line(&self) -> Option<Self> {
        if self.line == self.owner.borrow().len_lines() - 1 {
            None
        } else {
            Some(Self {
                owner: self.owner.clone(),
                line: self.line + 1,
            })
        }
    }

    pub fn start(&self) -> AbsoluteIndex {
        AbsoluteIndex(self.owner.borrow().line_to_byte(self.line))
    }

    pub fn end(&self) -> AbsoluteIndex {
        if self.line + 1 >= self.owner.borrow().len_lines() {
            AbsoluteIndex(self.owner.borrow().len_bytes())
        } else {
            AbsoluteIndex(prev_grapheme_boundary(
                &self.owner.borrow().slice(..),
                self.owner.borrow().line_to_byte(self.line + 1),
            ))
        }
    }

    pub fn len(&self) -> RelativeIndex {
        RelativeIndex(self.rope().len_bytes())
    }

    pub fn boundary(&self) -> Range<AbsoluteIndex> {
        self.start()..self.end()
    }

    pub fn relative_index_to_column(&self, index: RelativeIndex, tabsize: usize) -> Column {
        let mut col = 0;
        let mut i = 0;
        while i < index.0 {
            let c = self.rope().char(self.rope().byte_to_char(i));
            match c {
                ' ' => {
                    col += 1;
                    i += 1;
                }
                '\t' => {
                    let nb_space = tabsize - col % tabsize;
                    col += nb_space;
                    i += 1;
                }
                _ => {
                    i = next_grapheme_boundary(&self.rope().slice(..), i);
                    col += 1;
                }
            }
        }
        Column(col)
    }

    pub fn column_to_relative_index(&self, column: Column, tabsize: usize) -> RelativeIndex {
        let mut col = 0;
        let mut i = 0;
        while col < column.0 && i<self.len().0 {
            let c = self.rope().char(self.rope().byte_to_char(i));
            match c {
                ' ' => {
                    col += 1;
                    i += 1;
                }
                '\t' => {
                    let nb_space = tabsize - col % tabsize;
                    col += nb_space;
                    i += 1;
                }
                _ => {
                    i = next_grapheme_boundary(&self.rope().slice(..), i);
                    col += 1;
                }
            }
        }
        RelativeIndex(i)
    }

    pub fn column_to_absolute_index(&self, column: Column, tabsize: usize) -> AbsoluteIndex {
        self.relative_to_absolute_index(self.column_to_relative_index(column,tabsize))
    }

    pub fn absolute_index_to_column(&self, index: AbsoluteIndex, tabsize: usize) -> Column {
        self.relative_index_to_column(self.absolute_to_relative_index(index),tabsize)
    }

    pub fn relative_to_absolute_index(&self, rel_index: RelativeIndex) -> AbsoluteIndex {
        AbsoluteIndex(self.start().0 + rel_index.0)
    }

    pub fn absolute_to_relative_index(&self, abs_index: AbsoluteIndex) -> RelativeIndex {
        RelativeIndex(abs_index.0 - self.start().0)
    }

    pub fn visible_start(&self) -> RelativeIndex {
        // here we're assuming ' ' and '\t' are 1 byte length
        RelativeIndex(self.rope().chars().take_while(|c| *c != '\t' && *c != ' ').count())
    }

    pub fn get_displayable_string(&self, tabsize: usize, out: &mut String, index_conversion: &mut Vec<RelativeIndex>) {
        out.clear();
        index_conversion.clear();
        if self.line >= self.owner.borrow().len_lines() {
            return;
        }

        let mut index = 0;
        for c in self.owner.borrow().line(self.line).chars() {
            match c {
                ' ' => {
                    index_conversion.push(RelativeIndex(index));
                    out.push(' ');
                    index += 1;
                }
                '\t' => {
                    let nb_space = tabsize - index % tabsize;
                    index_conversion.push(RelativeIndex(index));
                    out.push_str(&" ".repeat(nb_space));
                    index += nb_space;
                }
                _ => {
                    out.push(c);
                    for i in index..index + c.len_utf8() {
                        index_conversion.push(RelativeIndex(index));
                    }
                    index += c.len_utf8();
                }
            }
        }
        index_conversion.push(RelativeIndex(index));
    }
}

impl Iterator for Line {
    type Item = Line;
    fn next(&mut self) -> Option<Self::Item> {
        self.next_line()
    }
}

/// Finds the previous grapheme boundary before the given char position.
pub fn prev_grapheme_boundary(slice: &RopeSlice, byte_idx: usize) -> usize {
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
pub fn next_grapheme_boundary(slice: &RopeSlice, byte_idx: usize) -> usize {
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

pub fn byte_to_line_boundary(slice: &RopeSlice, index: usize) -> Range<usize> {
    let line = slice.byte_to_line(index);
    line_boundary(slice, line)
}

pub fn line_boundary(slice: &RopeSlice, line: usize) -> Range<usize> {
    let line_start = slice.line_to_byte(line);
    let line_end = if line + 1 >= slice.len_lines() {
        slice.len_bytes()
    } else {
        prev_grapheme_boundary(slice, slice.line_to_byte(line + 1))
    };
    line_start..line_end
}

pub fn line_index_to_column(line_slice: &RopeSlice, index: usize, tabsize: usize) -> usize {
    let mut col = 0;
    let mut i = 0;
    while i < index {
        let c = line_slice.char(line_slice.byte_to_char(index));
        match c {
            ' ' => {
                col += 1;
                i += 1;
            }
            '\t' => {
                let nb_space = tabsize - col % tabsize;
                col += nb_space;
                i += 1;
            }
            _ => {
                i = next_grapheme_boundary(line_slice, i);
                col += 1;
            }
        }
    }
    col
}

pub fn line_indent_len(slice: &RopeSlice, line: usize, tabsize: usize) -> usize {
    let mut col = 0;
    for c in slice.line(line).chars() {
        match c {
            ' ' => {
                col += 1;
            }
            '\t' => {
                let nb_space = tabsize - col % tabsize;
                col += nb_space;
            }
            _ => {
                break;
            }
        }
    }
    col
}

pub fn byte_to_line_first_column(slice: &RopeSlice, index: usize) -> usize {
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

pub fn index_to_point(slice: &RopeSlice, index: usize) -> (usize, usize) {
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

pub fn point_to_index(slice: &RopeSlice, vcol: usize, line: usize) -> (usize, usize, usize) {
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
