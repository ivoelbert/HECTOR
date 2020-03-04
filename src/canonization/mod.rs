mod linearize;
mod basic_blocks;
mod trace_schedule;
use linearize::linearize;
use basic_blocks::basic_blocks;
use trace_schedule::trace_schedule;

use crate::tree::*;
use Tree::*;
use frame::Frame;

fn canonize(tree : Tree::Stm) -> Vec<Tree::Stm> {
    trace_schedule(basic_blocks(linearize(tree)))
}