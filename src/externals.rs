#[derive(Clone, Copy)]
pub enum ArgumentType {
    String,
    Int,
}

pub struct External {
    pub name: &'static str,
    pub is_runtime: bool,
    pub arguments: Vec<ArgumentType>,
    pub return_value: Option<ArgumentType>,
    // pub description: &'static str
}

lazy_static! {
    pub static ref EXTERNALS : Vec<External> = vec![
        External {
            name: "print",
            arguments: vec![ArgumentType::String],
            return_value: None,
            is_runtime: false
        },
        External {
            name: "getchar",
            arguments: vec![],
            return_value: Some(ArgumentType::String),
            is_runtime: false
        },
        External {
            name: "getstring",
            arguments: vec![],
            return_value: Some(ArgumentType::String),
            is_runtime: false
        },
        External {
            name: "ord",
            arguments: vec![ArgumentType::String],
            return_value: Some(ArgumentType::Int),
            is_runtime: false
        },
        External {
            name: "chr",
            arguments: vec![ArgumentType::Int],
            return_value: Some(ArgumentType::String),
            is_runtime: false
        },
        External {
            name: "size",
            arguments: vec![ArgumentType::String],
            return_value: Some(ArgumentType::Int),
            is_runtime: false
        },
        External {
            name: "substring",
            arguments: vec![ArgumentType::String, ArgumentType::Int, ArgumentType::Int],
            return_value: Some(ArgumentType::String),
            is_runtime: false
        },
        External {
            name: "concat",
            arguments: vec![ArgumentType::String, ArgumentType::String],
            return_value: Some(ArgumentType::String),
            is_runtime: false
        },
        External {
            name: "not",
            arguments: vec![ArgumentType::Int],
            return_value: Some(ArgumentType::Int),
            is_runtime: false
        },
        External {
            name: "exit",
            arguments: vec![ArgumentType::Int],
            return_value: None,
            is_runtime: false
        },

        External {
            name: "alloc_array",
            arguments: vec![ArgumentType::Int, ArgumentType::Int],
            return_value: Some(ArgumentType::Int),
            is_runtime: true
        },
        External {
            name: "alloc_record",
            arguments: vec![ArgumentType::Int],
            return_value: Some(ArgumentType::Int),
            is_runtime: true
        },
        External {
            name: "check_index_array",
            arguments: vec![ArgumentType::Int, ArgumentType::Int],
            return_value: None,
            is_runtime: true
        },
        External {
            name: "check_nil",
            arguments: vec![ArgumentType::Int],
            return_value: None,
            is_runtime: true
        },
        External {
            name: "str_equals",
            arguments: vec![ArgumentType::String, ArgumentType::String],
            return_value: Some(ArgumentType::Int),
            is_runtime: true
        },
        External {
            name: "str_not_equals",
            arguments: vec![ArgumentType::String, ArgumentType::String],
            return_value: Some(ArgumentType::Int),
            is_runtime: true
        },
        External {
            name: "str_less",
            arguments: vec![ArgumentType::String, ArgumentType::String],
            return_value: Some(ArgumentType::Int),
            is_runtime: true
        },
        External {
            name: "str_less_or_equals",
            arguments: vec![ArgumentType::String, ArgumentType::String],
            return_value: Some(ArgumentType::Int),
            is_runtime: true
        },
        External {
            name: "str_greater",
            arguments: vec![ArgumentType::String, ArgumentType::String],
            return_value: Some(ArgumentType::Int),
            is_runtime: true
        },
    ];
}