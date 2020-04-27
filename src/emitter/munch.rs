use super::*;
use Tree::Exp::*;
use Tree::Stm::*;

use std::collections::HashMap;
struct LocalEnv {
    table: HashMap<String, u32>,
    index: u32,
}

impl LocalEnv {
    pub fn new() -> Self {
        LocalEnv {
            table: HashMap::new(),
            index: 0
        }
    }
    pub fn insert(self: &mut Self, name: String) -> u32 {
        self.table.insert(name, self.index);
        self.index += 1;
        self.index - 1
    }
    pub fn get(self: &Self, name: &str) -> Option<u32> {
        self.table.get(name).copied()
    }
    pub fn finish(self: Self) -> Vec<elements::Local> {
        let locals = vec![];
        for i in 0..self.index {
            locals.push(elements::Local {
                value_type: elements::ValueType::I32,
                count: i
            })
        };
        locals
    }
}

fn get_global_index(name: &str) -> u32 {
    if name == STACK_POINTER {
        return 0;
    };
    if name == FRAME_POINTER {
        return 1;
    };
    if name == RETURN_VALUE {
        return 2;
    };
    panic!("should be the only globals")
}

pub fn munch_stm(stm: Tree::Stm, mut locals : LocalEnv) -> (Vec<elements::Instruction>, LocalEnv) {
	match stm {
		// TODO
		MOVE(to_exp, from_exp) => {
            let (mut from_code, locals) = munch_exp(*from_exp, locals);
            match *to_exp {
                MEM(dir) => {
                    from_code.push(I32Store(0, 0))
                },
                LOCAL(name) => {
                    let index = if let Some(index) = locals.get(&name) {
                        index
                    } else {
                        locals.insert(name)
                    };
                    from_code.push(SetLocal(0))
                },
                GLOBAL(global) => from_code.push(SetGlobal(0)),
                _ => panic!("canonization should delete this?")
            };
            (from_code, locals)
        },
		LABEL(Label) => {
            // Analizar por nombre del label
            // si es un done -> End
            // sino, Block?.
            // los indices son por nesting depth. dolor de ojete
            (vec![I32Const(32)], locals)
        },
		JUMP(label_exp, _) => vec![I32Const(32)],
		CJUMP(oper, left, right, t, f) => vec![I32Const(32)],
		EXP(exp) => munch_exp(*exp, locals),
		SEQ(_, _) => panic!("canonization should delete this"),
	}
}

pub fn munch_exp(exp: Tree::Exp, locals : LocalEnv) -> (Vec<elements::Instruction>, LocalEnv) {
	match exp {
		// TOOD
		BINOP(oper, left, right) => vec![I32Const(32)],
		CALL(label, args) => vec![I32Const(32)],
		NAME(label) => vec![I32Const(32)],
		LOCAL(local) => vec![I32Const(32)],
		GLOBAL(global) => vec![I32Const(32)],
		MEM(dir) => vec![I32Const(32)],
		CONST(i) => vec![I32Const(i)],
		ESEQ(_, _) => panic!("canonization should delete this"),
	};
	vec![I32Const(32)]

}

// pub fn munch()