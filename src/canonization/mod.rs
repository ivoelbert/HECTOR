mod linearize;
mod basic_blocks;
mod trace_schedule;

use serde::{Serialize};

use linearize::linearize;
use basic_blocks::basic_blocks;
use trace_schedule::trace_schedule;

use crate::tree::{Fragment, Tree};
use crate::tree::frame::Frame;
use crate::tree::level;

#[derive(Clone, Serialize)]
pub enum CanonFrag {
    Proc {
        body: Vec<Tree::Stm>,
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
                        body: trace_schedule(basic_blocks(linearize(body))),
                        frame
                    },
                Fragment::ConstString(l, s) => CanonFrag::ConstString(l, s)
            }
        })
        .collect()
}