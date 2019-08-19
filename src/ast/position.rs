use std::fmt::{self, Display, Debug, Formatter};
use std::u32;

#[derive(Clone, Copy)]
pub struct Pos {
    pub column: u32,
    pub line: u32,
}

impl Debug for Pos {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "line {}, column {}", self.line, self.column)
    }
}

impl Pos {
    pub fn new(line: u32, column: u32) -> Self {
        Self {
            column,
            line,
        }
    }
}

impl Display for Pos {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "{}:{}:", self.line, self.column)
    }
}

#[derive(Clone)]
pub struct WithPos<T> {
    pub node: T,
    pub pos: Pos,
}

impl<T> WithPos<T> {
    pub fn new(node: T, pos: Pos) -> Self {
        Self {
            node,
            pos,
        }
    }

    pub fn dummy(node: T) -> Self {
        Self {
            node,
            pos: Pos::new(u32::max_value(), u32::max_value()),
        }
    }
}

impl<T: PartialEq> PartialEq for WithPos<T> {
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node
    }
}