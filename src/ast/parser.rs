use super::tigerabs::{Exp, _Exp};
use super::position::{Pos, WithPos};

pub enum ParseError {
    UnexpectedToken(Pos),
}


pub fn parse<'a>(source : String) -> Result<Exp<'a>, ParseError> {
    return Ok(WithPos {
        node: _Exp::UnitExp,
        pos: Pos {
            line: 0,
            column: 0,
        }
    });
}
