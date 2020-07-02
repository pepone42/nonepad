use super::rope_utils::{next_grapheme_boundary, prev_grapheme_boundary};
use druid::Data;
use std::ops::Add;
use std::ops::{AddAssign, Sub, SubAssign};
use super::buffer::Buffer;

pub trait Position {
    fn absolute(&self, buffer: &Buffer) -> Absolute;
    fn point(&self, buffer: &Buffer) -> Point;
    fn line(&self, buffer: &Buffer) -> Line;
    fn up(&self, buffer: &Buffer) -> Self;
    fn down(&self, buffer: &Buffer) -> Self;
    //fn relative(&self, buffer: &Buffer) -> Relative;
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Data)]
pub struct Point {
    pub col: Column,
    pub line: Line,
    pub relative: Relative,
}

impl Position for Point {
    fn absolute(&self, buffer: &Buffer) -> Absolute {
        self.line.start(buffer) + self.relative
    }
    fn point(&self, _buffer: &Buffer) -> Point {
        *self
    }
    fn line(&self, _buffer: &Buffer) -> Line {
        self.line
    }

    fn up(&self, buffer: &Buffer) -> Self {
        let line = self.line(buffer).prev().unwrap_or(self.line);
        let col = if self.col > line.grapheme_len(buffer) {
            line.grapheme_len(buffer)
        } else {
            self.col
        };
        Self::new(col, line, buffer)
    }
    fn down(&self, buffer: &Buffer) -> Self {
        let line = self.line(buffer).next(buffer).unwrap_or(self.line);
        let col = if self.col > line.grapheme_len(buffer) {
            line.grapheme_len(buffer)
        } else {
            self.col
        };
        Self::new(col, line, buffer)
    }
}

impl Point {
    fn calc_relative(col: Column, line: Line, buffer: &Buffer) -> Relative {
        let mut c = 0;
        let mut i = Relative::from(0);
        let a = line.start(buffer);
        while c < col.index && i < line.byte_len(buffer) {
            let ch = buffer.char(a + i);
            match ch {
                ' ' => {
                    c += 1;
                    i += 1;
                }
                '\t' => {
                    let nb_space = buffer.tabsize - c % buffer.tabsize;
                    c += nb_space;
                    i += 1;
                }
                _ => {
                    i = next_grapheme_boundary(&buffer.line_slice(line), i).into();
                    c += 1;
                }
            }
        }
        i
    }

    fn calc_col(relative: Relative, line: Line, buffer: &Buffer) -> Column {
        let mut c = Column::from(0);
        let mut i = Relative::from(0);
        let a = line.start(buffer);// Absolute::from(rope.line_to_byte(line.index));
        while i < relative {
            let ch = buffer.char(a + i);
            match ch {
                ' ' => {
                    c += 1;
                    i += 1;
                }
                '\t' => {
                    let nb_space = buffer.tabsize - c.index % buffer.tabsize;
                    c += nb_space;
                    i += 1;
                }
                _ => {
                    i = next_grapheme_boundary(&buffer.line_slice(line), i).into();
                    c += 1;
                }
            }
        }
        c
    }

    pub fn new(col: Column, line: Line, buffer: &Buffer) -> Self {
        let line = if line.index >= buffer.len_lines() {
            buffer.len_lines().into()
        } else {
            line
        };
        Self {
            col,
            line,
            relative: Self::calc_relative(col, line, buffer),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Data)]
pub struct Absolute {
    pub index: usize,
}

impl From<usize> for Absolute {
    fn from(index: usize) -> Self {
        Self { index }
    }
}

impl Into<usize> for Absolute {
    fn into(self) -> usize {
        self.index
    }
}

impl AddAssign<Relative> for Absolute {
    fn add_assign(&mut self, rhs: Relative) {
        self.index += rhs.index;
    }
}

impl AddAssign<usize> for Absolute {
    fn add_assign(&mut self, rhs: usize) {
        self.index += rhs;
    }
}

impl SubAssign<Relative> for Absolute {
    fn sub_assign(&mut self, rhs: Relative) {
        self.index -= rhs.index;
    }
}

impl Add<Relative> for Absolute {
    type Output = Absolute;
    fn add(self, rhs: Relative) -> Self::Output {
        (rhs.index + self.index).into()
    }
}

impl Add<usize> for Absolute {
    type Output = Absolute;
    fn add(self, rhs: usize) -> Self::Output {
        (rhs + self.index).into()
    }
}

impl Sub<Absolute> for Absolute {
    type Output = Relative;
    fn sub(self, rhs: Absolute) -> Self::Output {
        Relative::from(self.index - rhs.index)
    }
}

impl Sub<Relative> for Absolute {
    type Output = Absolute;
    fn sub(self, rhs: Relative) -> Self::Output {
        Absolute::from(self.index - rhs.index)
    }
}

impl Sub<usize> for Absolute {
    type Output = Absolute;
    fn sub(self, rhs: usize) -> Self::Output {
        Absolute::from(self.index - rhs)
    }
}

impl Position for Absolute {
    fn absolute(&self, _buffer: &Buffer) -> Absolute {
        *self
    }
    fn point(&self, buffer: &Buffer) -> Point {
        let line = self.line(buffer);// buffer.absolute_to_line(*self);
        let relative = *self - line.start(buffer);
        Point {
            line,
            relative,
            col: Point::calc_col(relative, line, buffer),
        }
    }
    fn line(&self, buffer: &Buffer) -> Line {
        Line::from(buffer.absolute_to_line(*self))
    }
    fn up(&self, buffer: &Buffer) -> Self {
        self.point(buffer).up(buffer).absolute(buffer)
    }
    fn down(&self, buffer: &Buffer) -> Self {
        self.point(buffer).down(buffer).absolute(buffer)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Data)]
pub struct Relative {
    pub index: usize,
}

impl From<usize> for Relative {
    fn from(index: usize) -> Self {
        Self { index }
    }
}

impl Into<usize> for Relative {
    fn into(self) -> usize {
        self.index
    }
}

impl AddAssign<usize> for Relative {
    fn add_assign(&mut self, rhs: usize) {
        self.index += rhs;
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Data)]
pub struct Column {
    pub index: usize,
}

impl From<usize> for Column {
    fn from(index: usize) -> Self {
        Self { index }
    }
}

impl Into<usize> for Column {
    fn into(self) -> usize {
        self.index
    }
}

impl AddAssign<usize> for Column {
    fn add_assign(&mut self, rhs: usize) {
        self.index += rhs;
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Data)]
pub struct Line {
    pub index: usize,
}

impl From<usize> for Line {
    fn from(index: usize) -> Self {
        Self { index }
    }
}

impl Into<usize> for Line {
    fn into(self) -> usize {
        self.index
    }
}

impl Line {
    pub fn start(&self, buffer: &Buffer) -> Absolute {
        buffer.line_to_absolute(*self)
    }
    pub fn end(&self, buffer: &Buffer) -> Absolute {
        // TODO use self.next
        if self.index + 1 >= buffer.len_lines() {
            Absolute::from(buffer.len())
        } else {
            Absolute::from(prev_grapheme_boundary(
                &buffer.slice(..),
                buffer.line_to_absolute(self.index + 1),
            ))
        }
    }
    pub fn byte_len(&self, buffer: &Buffer) -> Relative {
        self.end(buffer) - self.start(buffer)
    }
    pub fn grapheme_len(&self, buffer: &Buffer) -> Column {
        let mut col = Column::from(0);
        let mut i = Relative::from(0);
        let a = Absolute::from(buffer.line_to_absolute(self.index));
        while i < self.byte_len(buffer) {
            let c = buffer.char(a + i);
            match c {
                ' ' => {
                    col += 1;
                    i += 1;
                }
                '\t' => {
                    let nb_space: usize = buffer.tabsize - col.index % buffer.tabsize;
                    col += nb_space;
                    i += 1;
                }
                _ => {
                    i = next_grapheme_boundary(&buffer.line_slice(*self), i).into();
                    col += 1;
                }
            }
        }
        col
    }
    pub fn prev(&self) -> Option<Self> {
        if self.index == 0 {
            None
        } else {
            Some(Self { index: self.index - 1 })
        }
    }
    pub fn next(&self, buffer: &Buffer) -> Option<Self> {
        if self.index == buffer.len_lines() - 1 {
            None
        } else {
            Some(Self { index: self.index + 1 })
        }
    }
    pub fn displayable_string(
        &self,
        buffer: &Buffer,
        tabsize: usize,
        out: &mut String,
        rel_to_byte: &mut Vec<Relative>,
        byte_to_rel: &mut Vec<Relative>,
    ) {
        out.clear();
        rel_to_byte.clear();
        byte_to_rel.clear();
        if self.index >= buffer.len_lines() {
            return;
        }

        let mut rel_index = 0;
        let mut byte_index = 0;
        for c in buffer.line_slice(*self).chars() {
            match c {
                ' ' => {
                    rel_to_byte.push(rel_index.into());
                    byte_to_rel.push(byte_index.into());
                    out.push(' ');
                    rel_index += 1;
                    byte_index += 1;
                }
                '\t' => {
                    let nb_space = tabsize - rel_index % tabsize;
                    rel_to_byte.push(rel_index.into());
                    for _ in 0..nb_space {
                        byte_to_rel.push(byte_index.into());
                    }
                    out.push_str(&" ".repeat(nb_space));
                    rel_index += nb_space;
                    byte_index += 1;
                }
                _ => {
                    out.push(c);
                    for _ in rel_index..rel_index + c.len_utf8() {
                        rel_to_byte.push(rel_index.into());
                        byte_to_rel.push(byte_index.into());
                    }
                    rel_index += c.len_utf8();
                    byte_index += c.len_utf8();
                }
            }
        }
        rel_to_byte.push(rel_index.into());
        byte_to_rel.push(byte_index.into());
    }
    pub fn iter<'r>(&self, buffer: &'r Buffer) -> LineIterator<'r> {
        LineIterator { buffer, line: *self }
    }
}

pub struct LineIterator<'r> {
    buffer: &'r Buffer,
    line: Line,
}

impl<'r> Iterator for LineIterator<'r> {
    type Item = Line;
    fn next(&mut self) -> Option<Self::Item> {
        self.line.next(self.buffer)
    }
}

impl SubAssign<usize> for Line {
    fn sub_assign(&mut self, rhs: usize) {
        if self.index > 0 {
            self.index -= rhs;
        }
    }
}

impl Sub<Line> for Line {
    type Output = Line;
    fn sub(self, rhs: Line) -> Self::Output {
        Line::from(self.index - rhs.index)
    }
}

impl Sub<&Line> for &Line {
    type Output = Line;
    fn sub(self, rhs: &Line) -> Self::Output {
        Line::from(self.index - rhs.index)
    }
}
