use super::*;

use std::collections::HashMap;
#[derive(Clone)]
pub struct LocalEnv {
    table: HashMap<String, u32>,
    index: u32,
    formals: u32
}

impl LocalEnv {
    pub fn from_frame(frame: &Frame) -> (Self, Vec<ValueType>) {
        let mut table = HashMap::new();
        let mut index = 0;
        let mut formals = 0;
        let mut value_types = vec![];
        for (name, _) in frame.formals() {
            table.insert(name, index);
            index += 1;
            formals += 1;
            value_types.push(ValueType::I32)
        };
        (LocalEnv {table, index, formals}, value_types)
    }
    pub fn insert(self: &mut Self, name: String) -> u32 {
        self.table.insert(name, self.index);
        self.index += 1;
        self.index - 1
    }
    pub fn get(self: &Self, name: &str) -> Option<u32> {
        self.table.get(name).copied()
    }
    pub fn finish(self: Self) -> Vec<Local> {
        let mut locals = vec![];
        for i in self.formals..self.index {
            locals.push(Local::new(i, ValueType::I32))
        };
        locals
    }
}

// pub struct FunctionEnv {
//     table: HashMap<Label, u32>,
//     index: u32,
// }

// impl FunctionEnv {
//     pub fn new() -> Self {
//         FunctionEnv {
//             table: HashMap::new(),
//             index: 0,
//         }
//     }

//     pub fn insert(self: &mut Self, label: Label) -> u32 {
//         self.table.insert(label, self.index);
//         self.index += 1;
//         self.index - 1
//     }

//     pub fn get(self: &Self, name: &str) -> Option<u32> {
//         self.table.get(name).copied()
//     }

//     pub fn get_last_index(self: &Self) -> u32 {
//         self.index - 1
//     }
// }

pub struct StringEnv {
    table: HashMap<Label, u32>,
    offset: u32,
}

impl StringEnv {
    pub fn new() -> Self {
        StringEnv {
            table: HashMap::new(),
            offset: 0,
        }
    }

    pub fn insert(mut self: Self, label: Label, string: &str) -> Self{
        let len : u32 = string.len().try_into().unwrap();
        self.offset = self.offset + len;
        self.table.insert(label, self.offset);
        self
    }

    pub fn get(self: &Self, name: &str) -> Option<u32> {
        self.table.get(name).copied()
    }
}

// TODO: strings should keep record of the memory offsets
pub type LabelEnv = HashMap<Label, u32>;

pub type FunctionEnv = HashMap<Label, u32>;