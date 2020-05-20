use crate::rope_utils::{next_grapheme_boundary, prev_grapheme_boundary};
use ropey::Rope;
use std::ops::Add;
use std::ops::{AddAssign, Sub, SubAssign};

pub trait Position {
    fn absolute(&self, rope: &Rope, tabsize: usize) -> Absolute;
    fn point(&self, rope: &Rope, tabsize: usize) -> Point;
    fn line(&self, rope: &Rope) -> Line;
    fn up(&self, rope: &Rope, tabsize: usize) -> Self;
    fn down(&self, rope: &Rope, tabsize: usize) -> Self;
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
    pub col: Column,
    pub line: Line,
    pub relative: Relative,
}

impl Position for Point {
    fn absolute(&self, rope: &Rope, tabsize: usize) -> Absolute {
        self.line.start(rope) + self.relative
    }
    fn point(&self, rope: &Rope, tabsize: usize) -> Point {
        *self
    }
    fn line(&self, rope: &Rope) -> Line {
        self.line
    }
    fn up(&self, rope: &Rope, tabsize: usize) -> Self {
        let line = self.line(rope).prev().unwrap_or(self.line);
        let col = if self.col > line.grapheme_len(rope, tabsize) {
            line.grapheme_len(rope, tabsize)
        } else {
            self.col
        };
        Self::new(col, line, rope, tabsize)
    }
    fn down(&self, rope: &Rope, tabsize: usize) -> Self {
        let line = self.line(rope).next(rope).unwrap_or(self.line);
        let col = if self.col > line.grapheme_len(rope, tabsize) {
            line.grapheme_len(rope, tabsize)
        } else {
            self.col
        };
        Self::new(col, line, rope, tabsize)
    }
}

impl Point {
    fn relative(col: Column, line: Line, rope: &Rope, tabsize: usize) -> Relative {
        let mut c = 0;
        let mut i = Relative::from(0);
        let a = Absolute::from(rope.line_to_byte(line.index));
        while c < col.index && i < line.byte_len(rope) {
            let ch = rope.char(rope.byte_to_char((a+i).index));
            match ch {
                ' ' => {
                    c += 1;
                    i += 1;
                }
                '\t' => {
                    let nb_space = tabsize - c % tabsize;
                    c += nb_space;
                    i += 1;
                }
                _ => {
                    i = next_grapheme_boundary(&rope.line(line.index), i).into();
                    c += 1;
                }
            }
        }
        i
    }

    fn col(relative: Relative, line: Line, rope: &Rope, tabsize: usize) -> Column {
        let mut c = Column::from(0);
        let mut i = Relative::from(0);
        let a = Absolute::from(rope.line_to_byte(line.index));
        while i < relative {
            let ch = rope.char(rope.byte_to_char((a+i).index));
            match ch {
                ' ' => {
                    c += 1;
                    i += 1;
                }
                '\t' => {
                    let nb_space = tabsize - c.index % tabsize;
                    c += nb_space;
                    i += 1;
                }
                _ => {
                    i = next_grapheme_boundary(&rope.line(line.index), i).into();
                    c += 1;
                }
            }
        }
        c
    }

    pub fn new(col: Column, line: Line, rope: &Rope, tabsize: usize) -> Self {
        Self {
            col,
            line,
            relative: Self::relative(col, line, rope, tabsize),
        }
    }

    pub fn up(&mut self, rope: &Rope, tabsize: usize) {
        if self.line > Line::from(0) {
            self.line -= 1;
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

impl Position for Absolute {
    fn absolute(&self, _rope: &Rope, _tabsize: usize) -> Absolute {
        *self
    }
    fn point(&self, rope: &Rope, tabsize: usize) -> Point {
        let line = Line::from(rope.byte_to_line(self.index));
        let relative = Relative::from(self.index - line.start(rope).index);
        Point {
            line,
            relative,
            col: Point::col(relative, line, rope, tabsize),
        }
    }
    fn line(&self, rope: &Rope) -> Line {
        Line::from(rope.byte_to_line(self.index))
    }
    fn up(&self, rope: &Rope, tabsize: usize) -> Self {
        self.point(rope, tabsize).up(rope, tabsize).absolute(rope, tabsize)
    }
    fn down(&self, rope: &Rope, tabsize: usize) -> Self {
        self.point(rope, tabsize).down(rope, tabsize).absolute(rope, tabsize)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
    pub fn start(&self, rope: &Rope) -> Absolute {
        rope.line_to_byte(self.index).into()
    }
    pub fn end(&self, rope: &Rope) -> Absolute {
        // TODO use self.next
        if self.index + 1 >= rope.len_lines() {
            Absolute::from(rope.len_bytes())
        } else {
            Absolute::from(prev_grapheme_boundary(
                &rope.slice(..),
                rope.line_to_byte((self.index + 1).into()),
            ))
        }
    }
    pub fn byte_len(&self, rope: &Rope) -> Relative {
        self.end(rope) - self.start(rope)
    }
    pub fn grapheme_len(&self, rope: &Rope, tabsize: usize) -> Column {
        let mut col = Column::from(0);
        let mut i = Relative::from(0);
        let a = Absolute::from(rope.line_to_byte(self.index));
        while i < self.byte_len(rope) {
            let c = rope.char(rope.byte_to_char((a+i).index));
            match c {
                ' ' => {
                    col += 1;
                    i += 1;
                }
                '\t' => {
                    let nb_space: usize = tabsize - col.index % tabsize;
                    col += nb_space;
                    i += 1;
                }
                _ => {
                    i = next_grapheme_boundary(&rope.line(self.index), i).into();
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
    pub fn next(&self, rope: &Rope) -> Option<Self> {
        if self.index == rope.len_lines() - 1 {
            None
        } else {
            Some(Self { index: self.index + 1 })
        }
    }
    pub fn displayable_string(
        &self,
        rope: &Rope,
        tabsize: usize,
        out: &mut String,
        index_conversion: &mut Vec<Relative>,
    ) {
        out.clear();
        index_conversion.clear();
        if self.index >= rope.len_lines() {
            return;
        }

        let mut index = 0;
        for c in rope.line(self.index).chars() {
            match c {
                ' ' => {
                    index_conversion.push(index.into());
                    out.push(' ');
                    index += 1;
                }
                '\t' => {
                    let nb_space = tabsize - index % tabsize;
                    index_conversion.push(index.into());
                    out.push_str(&" ".repeat(nb_space));
                    index += nb_space;
                }
                _ => {
                    out.push(c);
                    for i in index..index + c.len_utf8() {
                        index_conversion.push(index.into());
                    }
                    index += c.len_utf8();
                }
            }
        }
        index_conversion.push(index.into());
    }
    pub fn iter<'r>(&self, rope: &'r Rope) -> LineIterator<'r> {
        LineIterator { rope, line: *self }
    }
}

pub struct LineIterator<'r> {
    rope: &'r Rope,
    line: Line,
}

impl<'r> Iterator for LineIterator<'r> {
    type Item = Line;
    fn next(&mut self) -> Option<Self::Item> {
        self.line.next(self.rope)
    }
}

impl SubAssign<usize> for Line {
    fn sub_assign(&mut self, rhs: usize) {
        if self.index > 0 {
            self.index -= 1;
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
