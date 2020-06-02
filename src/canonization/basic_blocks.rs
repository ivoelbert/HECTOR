//! From a list of cleaned trees, produce a list of
//! basic blocks satisfying the following properties:
//! 1. and 2. as in linearize;
//! 3.  Every block begins with a LABEL;
//! 4.  A LABEL appears only at the beginning of a block;
//! 5.  Any JUMP or CJUMP is the last stm in a block;
//! 6.  Every block ends with a JUMP or CJUMP;
//! Also produce the "label" to which control will be passed
//! upon exit.

use super::*;
use level::{Label, unique_named_label, named_label};
// pub type Block = Vec<Tree::Stm>;

#[derive(Clone, Debug, Serialize)]
/// Basic Block
///
/// A block of instructions that begins with a labe and end with a jump,
/// meaning that they all execute with no internal branching.
pub struct Block {
    /// The block statements
    pub stms: Vec<Tree::Stm>,
    /// Label on the first statement
    pub label: Label,
    /// List of labels in the jump statement at the end of the block
    pub target: Vec<Label>
}

/// Transform a linear IR into Basic Blocks
pub fn basic_blocks(stms: Vec<Tree::Stm>) -> Vec<Block> {
    use Tree::Stm::*;
    use Tree::Exp::*;
    let done_label = named_label("done");
    // El fold nos da el ultimo bloque porque cuando se quedo sin instrucciones
    // Hay que meterle un jump a done
    let (mut blocks, mut last_block) : (Vec<Block>, Vec<Tree::Stm>) = stms.into_iter()
        .fold((vec![], vec![LABEL(unique_named_label("blockfirst"))]), |(mut blocks, mut this_block), stm| -> (Vec<Block>, Vec<Tree::Stm>) {
            // Dados:
            //    los bloques previos en blocks
            //    el progreso de este bloque en this_block
            //    el stm actual
            // continuar el bloque o terminarlo
            match stm {
                // Si hay un label, terminamos este bloque con un JUMP a ese label,
                // que empieza uno nuevo.
                LABEL(l) => {
                    this_block.push(JUMP(NAME(l.clone()), vec![l.clone()]));
                    blocks.push(Block {
                        label: if let LABEL(l) = this_block.first().expect("empty block") {
                            l.clone()
                        } else {
                            panic!("all blocks should start with a label")
                        },
                        target: vec![l.clone()],
                        stms: this_block,
                    });
                    let new_block = vec![LABEL(l)];
                    (blocks, new_block)

                },
                // Si hay un JUMP, terminamos este bloque y empezamos el nuevo con un label.
                JUMP(l, ls) => {
                    this_block.push(JUMP(l, ls.clone()));
                    blocks.push(Block {
                        label: if let LABEL(l) = this_block.first().expect("empty block") {
                            l.clone()
                        } else {
                            panic!("all blocks should start with a label")
                        },
                        target: ls,
                        stms: this_block,
                    });
                    let new_block = vec![LABEL(unique_named_label("newblock-jump"))];
                    (blocks, new_block)
                },
                CJUMP(oper, left, right, t, f) => {
                    this_block.push(CJUMP(oper, left, right, t.clone(), f.clone()));
                    blocks.push(Block {
                        label: if let LABEL(l) = this_block.first().expect("empty block") {
                            l.clone()
                        } else {
                            panic!("all blocks should start with a label")
                        },
                        target: vec![t, f],
                        stms: this_block,
                    });
                    let new_block = vec![LABEL(unique_named_label("newblock-cjump"))];
                    (blocks, new_block)
                },
                // Cualquier otra instruccion simplemente se agrega al bloque.
                stm @ EXP(..)
                | stm @ MOVE(..)
                | stm @ SEQ(..) => {
                    this_block.push(stm);
                    (blocks, this_block)
                },
                // Si no quedan mas instrucciones, metemos un JUMP(NAME(done_label))
            }
        });
    last_block.push(JUMP(NAME(done_label.clone()), vec![done_label.clone()]));
    blocks.push(Block {
        label: if let LABEL(l) = last_block.first().expect("empty block") {
            l.clone()
        } else {
            panic!("all blocks should start with a label")
        },
        target: vec![done_label],
        stms: last_block,
    });
    blocks
}