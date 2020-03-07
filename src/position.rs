use ropey::Rope;

pub trait Position {
    fn as_usize(&self) -> usize;
    fn absolute(&self, rope: &Rope, tabsize: usize) -> Absolute;
    fn point(&self, rope: &Rope, tabsize: usize) -> Point;
}

pub struct Point {
    col: Column,
    line: Line,
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

impl Position for Absolute {
    fn as_usize(&self) -> usize {
        self.index
    }
    fn absolute(&self, rope: &Rope, tabsize: usize) -> Absolute {
        *self
    }
    fn point(&self, rope: &Rope, tabsize: usize) -> Point { unimplemented!() }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Column {
    index: usize,
}

impl From<usize> for Column {
    fn from(index: usize) -> Self {
        Self { index }
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
