pub type Label = String;
pub type Temp = String;
extern crate uid;

// unsafe? Is there a pure way?
// YES - use unique ids again.
// But, not sure how to make string from uid.
pub fn newtemp() -> Temp {
    String::from("Not_implemented")
}

pub fn newlabel() -> Label {
    String::from("Not_implemented")
}