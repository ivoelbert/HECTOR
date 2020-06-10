///! This module defines the external functions available to tiger code.




/// A function from the tiger runtime.
struct External {
    /// The label to call the function
    module:&'static str,
    label: &'static str,
    params: Vec<elements::ValueType>,
    return_type: Option<elements::ValueType>,
}

lazy_static! {
    static ref STANDARD_LIBRARY : Vec<Std> = [
        Std {
            name: "print",
            formals: vec![Arc::new(TString)],
            result: Arc::new(TUnit),
        }
            "print",
            "flush",
            "getchar",
            "getchar",
            "ord",
            "chr",
            "size",
            "substring",
            "concat",
            "not",
            "exit",
            // Runtime functions are preceded by a + to avoid collision with user-defined functions/variables.
            "+alloc_array",
            "+alloc_record",
            "+check_index_array",
            "+check_nil",
            "+str_equals",
            "+str_not_equals",
            "+str_less",
            "+str_less_or_equals",
            "+str_greater",
            "+str_greater_or_equals"
    ];
}

/// Generate a `ValueEnv` that contains functions from the tiger standard library and the runtime.
fn initial_value_env() -> ValueEnviroment {
    use TigerType::*;
    use EnvEntry::*;
    let mut value_env = ValueEnviroment::new();
    value_env.insert(Symbol::from("print"), Func {
        formals: vec![Arc::new(TString)],
        result: Arc::new(TUnit),
    });
    value_env.insert(Symbol::from("flush"), Func {
        formals: vec![],
        result: Arc::new(TUnit),
    });
    value_env.insert(Symbol::from("getchar"), Func {
        formals: vec![],
        result: Arc::new(TString),
    });
    value_env.insert(Symbol::from("ord"), Func {
        formals: vec![Arc::new(TString)],
        result: Arc::new(TInt(R::RW)),
    });
    value_env.insert(Symbol::from("chr"), Func {
        formals: vec![Arc::new(TInt(R::RW))],
        result: Arc::new(TString),
    });
    value_env.insert(Symbol::from("size"), Func {
        formals: vec![Arc::new(TString)],
        result: Arc::new(TInt(R::RW)),
    });
    value_env.insert(Symbol::from("substring"), Func {
        formals: vec![Arc::new(TString)],
        result: Arc::new(TString),
    });
    value_env.insert(Symbol::from("concat"), Func {
        formals: vec![Arc::new(TString), Arc::new(TString)],
        result: Arc::new(TString),
    });
    value_env.insert(Symbol::from("not"), Func {
        formals: vec![Arc::new(TInt(R::RW))],
        result: Arc::new(TInt(R::RW)),
    });
    value_env.insert(Symbol::from("exit"), Func {
        formals: vec![Arc::new(TInt(R::RW))],
        result: Arc::new(TUnit),
    });
    value_env
}

pub fn initial_translation_value_env() -> ValueEnviroment {
    // let externals = [
    //     "print",
    //     "flush",
    //     "getchar",
    //     "getchar",
    //     "ord",
    //     "chr",
    //     "size",
    //     "substring",
    //     "concat",
    //     "not",
    //     "exit",
    //     // Runtime functions are preceded by a + to avoid collision with user-defined functions/variables.
    //     "+alloc_array",
    //     "+alloc_record",
    //     "+check_index_array",
    //     "+check_nil",
    //     "+str_equals",
    //     "+str_not_equals",
    //     "+str_less",
    //     "+str_less_or_equals",
    //     "+str_greater",
    //     "+str_greater_or_equals"
    // ];
    let externals = EXTERNALS
        .iter()
        .map(|External {name, is_runtime, ..}| name.to_string())
        .collect::<Vec<String>();
    externals.iter().map(|name| -> (String, EnvEntry) {
        (name.to_string(), EnvEntry::Func {
            label: named_label(name),
            external: true,
            depth: 0
        })
    }).collect()
}