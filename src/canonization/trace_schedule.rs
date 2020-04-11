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
    // console_log!("table: {:?}", &table);
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
                (maybe_true_block, Some(false_block)) if !false_block.is_empty() => {
                    // console_log!("caso false block: {:?}", &false_block);
                    // console_log!("table: {:?}", &table);
                    // console_log!("this_block: {:?}", &most);
                    // console_log!("rest: {:?}", &rest);
                    if let Some(true_block) = maybe_true_block {
                        table.insert(t, true_block);
                    }
                    let false_block_trace = trace(table, false_block, rest);
                    // console_log!("false_block_trace {:?}", &false_block_trace);
                    vec![this_block, false_block_trace].concat()
                }
                (Some(true_block), maybe_false_block) if !true_block.is_empty() => {
                    let neg_jump = CJUMP(not_rel(o), a, b, t, f.clone());
                    // console_log!("caso true block: {:?}", &true_block);
                    // console_log!("most: {:?}", most);
                    if let Some(false_block) = maybe_false_block {
                        table.insert(f, false_block);
                    }
                    let true_block_trace = trace(table, true_block, rest);
                    // console_log!("true_block trace {:?}", &true_block_trace);
                    vec![most, vec![neg_jump], true_block_trace].concat()
                },
                _ => {
                    // TODO: esto capaz se rompe y no aparece ninguno de los dos
                    let new_false = level::unique_named_label("-false");
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
        // console_log!("get_next {:?}", &first_block);
        let first_block_label = get_block_label(&first_block);
        // console_log!("get_next table res {:?}", &table.get(&first_block_label));
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

    let res = vec![get_next(table, basic_blocks), vec![LABEL(done_label)]].concat();
    // console_log!("trace_schedule: {:?}", &res);
    res
}