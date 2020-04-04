use super::*;
use level::Label;
use std::collections::HashMap;
use Tree::Stm::*;
use Tree::Exp::*;
use Tree::not_rel;

type Block = Vec<Tree::Stm>;
type BlockTable = HashMap<Label, Block>;

fn get_block_label(block: &Block) -> Label {
    let block_first_statement = block.first().expect("No empty blocks");
    let label = if let LABEL(l) = block_first_statement {
        l
    } else {
        panic!("All blocks should start with a label")
    };
    label.clone()
}

pub fn trace_schedule((basic_blocks, done_label): (Vec<Block>, Label)) -> Vec<Tree::Stm> {
    // use Tree::Exp::*;
    // Armamos una tabla con todos los bloques indexados por label
    let table : BlockTable = basic_blocks
        .iter()
        .map(|block| -> (Label, Block) {
            (get_block_label(block), block.clone())
        })
        .collect();

    fn splitlast(mut block: Block) -> (Block, Tree::Stm) {
        let last = block.pop().expect("No empty blocks");
        (block, last)
    }
    // Si no hay mas bloques, paramos
    // Si hay un bloque que no empieza con Label, panico.
    // Para cada bloque
    //      Si esta en la tabla, armamos la traza.
        //      Metemos el bloque en la tabla con un nil.
        //      Tomamos el ultimo stm.
        //      Si es un JUMP(NAME(..)), buscamos el label en la tabla:
        //          Si esta con un bloque nil (los metimos como bandera) o no esta, cerramos la traza y la concatenamos
        //          a resolver recursivamente el resto de las trazas. Va todo flat en una sola lista.
        //          Si esta con un bloque de verdad, la traza es el resto del bloque + la traza recursiva del bloque target
        //      Si es un JUMP de otra cosa, solo lo appendamos?
        //      Si es un CJUMP, buscamos ambos labels en la tabla:
        //          Si encuentro la falsa, uso esa como en JUMP.
        //          Si encuentro solo la verdadera, rearmo el bloque negando la condicion del CJUMP, dando vuelta las labels
        //          y ahi actuao como con la falsa.
        //      Si ninguna esta con un bloque no nil:
        //          - Armo un label false_nuevo
        //          - Cierro el bloque con el CJUMP con el false_nuevo
        //          - Meto un LABEL(false_nuevo) y un JUMP(false_viejo) en el medio
        //          - Sigo la traza recursivamente
    fn trace(mut table: BlockTable, this_block: Block, rest: Vec<Block>) -> Vec<Tree::Stm> {
        let (most, last) = splitlast(this_block.clone());
        match last {
            JUMP(jump_target, ..) => match &jump_target {
                NAME(jump_target_label) => {
                    match table.remove(jump_target_label) {
                        Some(target_block) if !target_block.is_empty() => {
                            vec![most, trace(table, target_block, rest)].concat()
                        },
                        _ => vec![this_block, get_next(table, rest)].concat()
                    }
                },
                _ => vec![this_block, get_next(table, rest)].concat(),
            }
            CJUMP(o, a, b, t, f) => match (table.remove(&t), table.remove(&f)) {
                (_, Some(false_block)) if !false_block.is_empty() => {
                    vec![this_block, trace(table, false_block, rest)].concat()
                }
                (Some(true_block), _) if !true_block.is_empty() => {
                    let neg_jump = CJUMP(not_rel(o), a, b, t, f);
                    vec![most, vec![neg_jump], trace(table, true_block, rest)].concat()
                },
                _ => {
                    let new_false = level::newlabel();
                    vec![
                        most,
                        vec![
                            CJUMP(o, a, b, t, new_false.clone()),
                            LABEL(new_false),
                            JUMP(NAME(f.clone()), vec![f])],
                        get_next(table, rest)
                    ].concat()
                }
            }
            _ => panic!("All blocks should end with JUMP or CJUMP")
        }
    }

    fn get_next(table: BlockTable, mut blocks: Vec<Block>) -> Vec<Tree::Stm> {
        let first_block = if !blocks.is_empty() {
            blocks.remove(0)
        } else {
            return vec![]
        };
        let first_block_label = get_block_label(&first_block);
        match table.get(&first_block_label) {
            Some(block) => {
                if block.is_empty() {
                    get_next(table, blocks)
                } else {
                    trace(table, first_block, blocks)
                }
            },
            None => get_next(table, blocks)
        }
    }

    vec![get_next(table, basic_blocks), vec![LABEL(done_label)]].concat()
}