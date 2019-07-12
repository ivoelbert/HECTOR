use super::tigerabs::{Exp, _Exp};
use super::position::{Pos, WithPos};

pub enum ParseError {
    UnexpectedToken(Pos),
}


pub fn parse(source : String) -> Result<Exp, ParseError> {
    return Ok(WithPos {
        node: _Exp::UnitExp,
        pos: Pos {
            line: 0,
            column: 0,
        }
    });
}
