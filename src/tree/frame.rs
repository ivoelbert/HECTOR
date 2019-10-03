use super::{Label, Access};

trait Frame {
    fn new_frame(name: Label, formals: Vec<bool>) -> Self;
    fn frame_name(frame: &Self) -> Label;
    fn alloc_local(frame: &mut Self, local: bool) -> Access;
    
}