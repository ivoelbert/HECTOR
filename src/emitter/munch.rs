use super::*;
use Tree::Exp::*;
use Tree::Stm::*;


fn get_global_index(name: &str) -> u32 {
    match name {
        n if n == STACK_POINTER => 0,
        n if n == FRAME_POINTER => 1,
        n if n == RETURN_VALUE => 2,
        _ => panic!("should be the only globals")
    }
}


fn wasm_binop(oper: Tree::BinOp) -> Instruction {
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
        LT => I32LtS,
        GT => I32GtS,
        LE => I32LeS,
        GE => I32GeS,
        ULT => I32LtU,
        ULE => I32LeU,
        UGT => I32GtU,
        UGE => I32GeU
    }
}



pub fn munch_stm(stm: Tree::Stm, locals : LocalEnv, functions: &FunctionEnv) -> (Vec<Instruction>, LocalEnv) {
	match stm {
		MOVE(to_exp, from_exp) => {
            let (value_code, mut locals) = munch_exp(*from_exp, locals, functions);
            match *to_exp {
                MEM(addr) => {
                    let (addr_code, locals) = munch_exp(*addr, locals, functions);
                    (vec![
                        addr_code,
                        vec![I32Const(WORD_SIZE), I32Mul],
                        value_code,
                        vec![I32Store(0, 0)], // CHEQUEAR: no se que son estos parametros
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
		LABEL(label) => {
            // TODO
            // Analizar por nombre del label
            // si es un done -> End
            // sino, Block?.
            // los indices son por nesting depth. dolor de ojete
            // Posiblemente hay que llevar una tabla de label -> depth o algo asi?
            match label {
                // TODO: epilogue here
                n if n.starts_with("-done") => (vec![], locals),
                _ => (vec![Block(BlockType::NoResult)], locals),
            }
        },
        JUMP(NAME(label), _) => {
            // TODO
            match label {
                // TODO: prologue here
                n if n.starts_with("-done") => (vec![End, GetGlobal(get_global_index(RETURN_VALUE)), End], locals),
                _ => {
                    // TODO
                    (vec![End], locals)
                },
            }
        },
        JUMP(_, _) => panic!("why?"),
		CJUMP(oper, left, right, t, f) => {
            // TODO
            (vec![I32Const(32)], locals)
        },
		EXP(exp) => munch_exp(*exp, locals, functions),
		SEQ(_, _) => panic!("canonization should delete this"),
	}
}

pub fn munch_exp(exp: Tree::Exp, locals : LocalEnv, functions: &FunctionEnv) -> (Vec<Instruction>, LocalEnv) {
	match exp {
		BINOP(oper, left, right) => {
            let (left_code, locals) = munch_exp(*left, locals, functions);
            let (right_code, locals) = munch_exp(*right, locals, functions);
            (vec![
                left_code,
                right_code,
                vec![wasm_binop(oper)]
            ].concat(), locals)
        },
		CALL(label_exp, args) => match *label_exp {
            NAME(label) => {
                let index = functions.get(label.as_ref()).unwrap();
                let args_code = args
                    .into_iter()
                    .map(|arg| -> Vec<Instruction> {
                        munch_exp(arg, locals.clone(), functions).0
                    }).collect::<Vec<Vec<Instruction>>>().concat();
                (vec![
                    args_code,
                    vec![Call(index)],
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
		ESEQ(_, _) => panic!("canonization should delete this"),
		MEM(_) => panic!("should be munched by bigger patern"),
		NAME(_) => panic!("should be munched by bigger patern"),
	}

}