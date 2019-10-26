pub use super::frame::{Frame};
extern crate uid;
pub type Label = uid::Id<u16>;
type LocalTemp = uid::Id<u16>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Temp {
    FRAME_POINTER,
    // Other special temps.
    Local(LocalTemp),
}
// unsafe? Is there a pure way?
// YES - use unique ids again.
// But, not sure how to make string from uid.
pub fn newtemp() -> Temp {
    //String::from("Not_implemented")
    Temp::Local(LocalTemp::new())
}

pub fn newlabel() -> Label {
    //String::from("Not_implemented")
    Label::new()
}

pub type LevelId = uid::Id<u16>;

#[derive(Clone, Debug)]
pub struct Level {
    pub frame: Frame,
    pub nesting_depth: i64,
    pub id: LevelId,
}