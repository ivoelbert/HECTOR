mod linearize;
mod basic_blocks;
// mod trace_schedule;

use serde::{Serialize};

use linearize::linearize;
use basic_blocks::basic_blocks;
pub use basic_blocks::Block;
// use trace_schedule::trace_schedule;

use crate::tree::{Fragment, Tree};
use crate::tree::frame::Frame;
use crate::tree::level;

#[derive(Clone, Serialize, Debug)]
pub enum CanonFrag {
    Proc {
        body: Vec<Block>,
        frame: Frame
    },
    ConstString(level::Label, String)
}

pub fn canonize(frags: Vec<Fragment>) -> Vec<CanonFrag> {
    frags
        .into_iter()
        .map(|frag| {
            match frag {
                Fragment::Proc{body, frame} =>
                    CanonFrag::Proc{
                        body: basic_blocks(linearize(body)),
                        frame
                    },
                Fragment::ConstString(l, s) => CanonFrag::ConstString(l, s)
            }
        })
        .collect()
}

