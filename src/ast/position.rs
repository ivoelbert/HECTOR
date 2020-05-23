//! Position in the source code for an AST component

use std::fmt::{self, Display, Debug, Formatter};
use serde::Serialize;
use std::u32;

#[derive(Clone, Copy, PartialEq, Eq, Serialize)]
/// Position in the source code for an AST component
pub struct Pos {
    /// The column number
    pub column: u32,
    /// The line number
    pub line: u32,
}

impl Debug for Pos {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "line {}, column {}", self.line, self.column)
    }
}

impl Pos {
    /// Create a new Pos
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
