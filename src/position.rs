use crate::rope_utils::next_grapheme_boundary;
use ropey::Rope;
use std::ops::Add;
use std::ops::AddAssign;

pub trait Position {
    fn absolute(&self, rope: &Rope, tabsize: usize) -> Absolute;
    fn point(&self, rope: &Rope, tabsize: usize) -> Point;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
    col: Column,
    line: Line,
    relative: Relative,
}

impl Position for Point {
    fn absolute(&self, rope: &Rope, tabsize: usize) -> Absolute {
        self.line.start(rope) + self.relative
    }
    fn point(&self, rope: &Rope, tabsize: usize) -> Point {
        *self
    }
}

impl Point {
    fn relative(col: Column, line: Line, rope: &Rope, tabsize: usize) -> Relative {
        let mut c = 0;
        let mut i = Relative::from(0);
        while c < col.index && i < line.len(rope) {
            let ch = rope.char(rope.byte_to_char(i.into()));
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
                    i = next_grapheme_boundary(&rope.slice(..), i).into();
                    c += 1;
                }
            }
        }
        i
    }

    fn col(relative:Relative, line: Line, rope: &Rope, tabsize: usize) -> Column {
        let mut c = Column::from(0);
        let mut i = Relative::from(0);
        while i < relative {
            let ch = rope.char(rope.byte_to_char(i.into()));
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
                    i = next_grapheme_boundary(&rope.slice(..), i).into();
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Absolute {
    index: usize,
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
            col: Point::col(relative,line,rope,tabsize)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Relative {
    index: usize,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Column {
    index: usize,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Line {
    index: usize,
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

impl Add<Relative> for Absolute {
    type Output = Absolute;
    fn add(self, rhs: Relative) -> Self::Output {
        (rhs.index + self.index).into()
    }
}

impl Line {
    pub fn start(&self, rope: &Rope) -> Absolute {
        rope.line_to_byte(self.index).into()
    }
    pub fn len(&self, rope: &Rope) -> Relative {
        Relative::from(rope.len_bytes())
    }
}
