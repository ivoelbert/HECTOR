use super::*;
use level::{Label, unique_named_label};


pub fn basic_blocks(stms: Vec<Tree::Stm>) -> (Vec<Vec<Tree::Stm>>, Label) {
    use Tree::Stm::*;
    use Tree::Exp::*;
    let done_label = unique_named_label("-done");
    // El fold nos da el ultimo bloque porque cuando se quedo sin instrucciones
    // Hay que meterle un jump a done
    let (mut blocks, mut last_block) : (Vec<Vec<Tree::Stm>>, Vec<Tree::Stm>) = stms.into_iter()
        .fold((vec![], vec![LABEL(unique_named_label("-blockfirst"))]), |(mut blocks, mut this_block), stm| -> (Vec<Vec<Tree::Stm>>, Vec<Tree::Stm>) {
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
                    blocks.push(this_block);
                    let new_block = vec![LABEL(l)];
                    (blocks, new_block)

                },
                // Si hay un JUMP, terminamos este bloque y empezamos el nuevo con un label.
                JUMP(l, ls) => {
                    this_block.push(JUMP(l, ls));
                    blocks.push(this_block);
                    let new_block = vec![LABEL(unique_named_label("-newblock-jump"))];
                    (blocks, new_block)
                },
                CJUMP(o, a, b, t, f) => {
                    this_block.push(CJUMP(o, a, b, t, f));
                    blocks.push(this_block);
                    let new_block = vec![LABEL(unique_named_label("-newblock-cjump"))];
                    (blocks, new_block)
                },
                // Cualquier otra instruccion simplemente se agrega al bloque.
                stm => {
                    this_block.push(stm);
                    (blocks, this_block)
                },
                // Si no quedan mas instrucciones, metemos un JUMP(NAME(done_label))
            }
        });
    last_block.push(JUMP(NAME(done_label.clone()), vec![done_label.clone()]));
    blocks.push(last_block);
    (blocks, done_label)
}