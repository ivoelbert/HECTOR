#![allow( clippy::module_name_repetitions)]

use super::*;
use Tree::Exp::*;
use Tree::Stm::*;

static NEXT_JUMP : &str = "nj";

fn get_global_index(name: &str) -> u32 {
    match name {
        n if n == STACK_POINTER => 0,
        n if n == FRAME_POINTER => 1,
        n if n == RETURN_VALUE => 2,
        n if n == NEXT_JUMP => 3,
        _ => panic!("should be the only globals")
    }
}


fn wasm_binop(oper: &Tree::BinOp) -> Instruction {
    use Tree::BinOp::*;
    match oper {
        PLUS => I32Add,
        MINUS => I32Sub,
        MUL => I32Mul,
        DIV => I32DivU,
        AND => I32And,
        OR => I32Or,
        LSHIFT => I32Shl,
        RSHIFT => I32ShrU,
        XOR => I32Xor,
        EQ => I32Eq,
        NE => I32Ne,
        LT => I32LtU,
        GT => I32GtS,
        LE => I32LeS,
        GE => I32GeS,
        ULT => I32LtU,
        ULE => I32LeU,
        UGT => I32GtU,
        UGE => I32GeU
    }
}



fn munch_stm(stm: Tree::Stm, locals : LocalEnv, labels: &LabelEnv, functions: &FunctionEnv, strings: &StringEnv, block_index: u32) -> (Vec<Instruction>, LocalEnv) {
	match stm {
		MOVE(to_exp, from_exp) => {
            let (value_code, mut locals) = munch_exp(*from_exp, locals, functions, strings);
            match *to_exp {
                MEM(addr) => {
                    let (addr_code, locals) = munch_exp(*addr, locals, functions, strings);
                    (vec![
                        addr_code,
                        vec![I32Const(WORD_SIZE), I32Mul],
                        value_code,
                        vec![I32Store(0, strings.offset)], // CHEQUEAR: no se que son estos parametros
                    ].concat(), locals)

                },
                LOCAL(name) => {
                    let index = if let Some(index) = locals.get(&name) {
                        index
                    } else {
                        locals.insert(name)
                    };
                    (vec![
                        value_code,
                        vec![SetLocal(index)]
                    ].concat(), locals)
                },
                GLOBAL(global) =>
                    (vec![
                        value_code,
                        vec![SetGlobal(get_global_index(&global))]
                    ].concat(), locals),
                _ => panic!("canonization should delete this?")
            }
        },
		LABEL(_label) => {
            // TODO: epilogue en done?
            // match label {
            //     n if n.starts_with("done") => (vec![], locals),
            //     _ => (vec![Block(BlockType::NoResult)], locals),
            // }
            (vec![], locals)
        },
        JUMP(NAME(label), _) => {
                // TODO: epilogue here?
                if label.starts_with("done") {
                    (vec![GetGlobal(get_global_index(RETURN_VALUE)), Return, End], locals) // TODO: esto esta en cualquiera
                } else {
                    let index = *labels.get(&label).unwrap();
                    (vec![
                        I32Const(i32::try_from(index).unwrap()),
                        SetGlobal(get_global_index(NEXT_JUMP)),
                        Br(block_index -1),
                        End, // Block
                    ],
                    locals)
                }
        },
        JUMP(_, _) => panic!("why?"),
		CJUMP(oper, left, right, t, f) => {
            let (left, locals) = munch_exp(*left, locals, functions, strings);
            let (right, locals) = munch_exp(*right, locals, functions, strings);
            let t_index = *labels.get(&t).unwrap();
            let f_index = *labels.get(&f).unwrap();
            (vec![
                left,
                right,
            vec![
                wasm_binop(&oper),
                If(BlockType::NoResult),
                    // CHEQUEAR: esto asume que un CJUMP nunca te puede mandar a done
                    I32Const(i32::try_from(t_index).unwrap()),
                    SetGlobal(get_global_index(NEXT_JUMP)),
                    Br(block_index),
                Else,
                    I32Const(i32::try_from(f_index).unwrap()),
                    SetGlobal(get_global_index(NEXT_JUMP)),
                    Br(block_index),
                End,
                End, // Block
                ]
            ].concat(),
            locals)
        },
		EXP(exp) => munch_exp(*exp, locals, functions, strings),
		SEQ(_, _) => panic!("canonization should delete this"),
	}
}

pub fn munch_exp(exp: Tree::Exp, locals : LocalEnv, functions: &FunctionEnv, strings: &StringEnv) -> (Vec<Instruction>, LocalEnv) {
	match exp {
		BINOP(oper, left, right) => {
            let (left_code, locals) = munch_exp(*left, locals, functions, strings);
            let (right_code, locals) = munch_exp(*right, locals, functions, strings);
            (vec![
                left_code,
                right_code,
                vec![wasm_binop(&oper)]
            ].concat(), locals)
        },
		CALL(label_exp, args) => match *label_exp {
            NAME(label) => {
                let index = functions.get(&label).unwrap();
                let args_code = args
                    .into_iter()
                    .map(|arg| -> Vec<Instruction> {
                        munch_exp(arg, locals.clone(), functions, strings).0
                    }).collect::<Vec<Vec<Instruction>>>().concat();
                (vec![
                    args_code,
                    vec![Call(*index)],
                ].concat(), locals)
            }
            _ => panic!("should not happen")
        },
		LOCAL(local) => {
            let index = locals.get(&local).expect("no unset local should be called");
            (vec![GetLocal(index)], locals)
        },
		GLOBAL(global) => (vec![GetGlobal(get_global_index(&global))], locals),
		CONST(i) => (vec![I32Const(i)], locals),
		NAME(label) => {
            // String
            let index = strings.get(&label).unwrap();
            (vec![I32Const((index).try_into().unwrap())], locals)
        },
        MEM(offset_exp) => {
            let (offset, locals) = munch_exp(*offset_exp, locals, functions, strings);
            (vec![
                offset,
            vec![
                I32Const(WORD_SIZE),
                I32Mul,
                I32Load(0, strings.offset)]
            ].concat(),
            locals)
        },
		ESEQ(_, _) => panic!("canonization should delete this"),
	}

}

pub fn munch_block(block: Block, locals : LocalEnv, labels: &LabelEnv, functions: &FunctionEnv, strings: &StringEnv, block_index: u32) -> (Vec<Instruction>, LocalEnv) {
    block.stms.into_iter()
    .fold((vec![], locals), |(mut instructions, locals): (Vec<Instruction>, LocalEnv), stm: Tree::Stm| -> (Vec<Instruction>, LocalEnv) {
        let (mut ins, locals) = munch_stm(stm, locals, labels, functions, strings, block_index);
        instructions.append(&mut ins);
        (instructions, locals)
    })
}

pub fn munch_body(blocks: Vec<Block>, locals : LocalEnv, functions: &FunctionEnv, strings: &StringEnv) -> (Vec<Instruction>, LocalEnv) {
    let block_instructions : Vec<Instruction>= blocks.iter().map(|_| Block(BlockType::NoResult)).collect();
    let first_block_index : i32 = (blocks.len() + 1).try_into().unwrap();
    let labels : LabelEnv = blocks
        .iter()
        .enumerate()
        .map(|(i, block)| (block.label.clone(), u32::try_from(first_block_index).unwrap() - u32::try_from(i).unwrap()))
        .collect();
    // console_log!("{:#?}", labels);
    let (body, locals) = blocks.into_iter()
        .fold((vec![], locals), |(mut instructions, locals): (Vec<Instruction>, LocalEnv), block: Block| -> (Vec<Instruction>, LocalEnv) {
            let block_index = *labels.get(&block.label).unwrap();
            let (mut ins, locals) = munch_block(block, locals, &labels, functions, strings, block_index);
            instructions.append(&mut ins);
            (instructions, locals)
        });
    let mut table = std::ops::Range { start: 1, end: u32::try_from(first_block_index).unwrap() + 1}
        .collect::<Box<[u32]>>();
    table.reverse();
    (vec![
    vec![
        I32Const(first_block_index), // First block
        SetGlobal(get_global_index(NEXT_JUMP)),
        Loop(BlockType::Value(ValueType::I32)),
    ],
        block_instructions, // body
    vec![
        Block(BlockType::NoResult),
            GetGlobal(get_global_index(NEXT_JUMP)),
            BrTable(Box::new(BrTableData{
                table,
                default: 0,
            })),
        End,
    ],
        body,
    vec![
        GetGlobal(get_global_index(RETURN_VALUE)),
        End, // Loop
        End, // function?
    ]]
    .concat(), locals)
}