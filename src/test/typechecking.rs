extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

use std::fs::{read_dir, read_to_string};
use std::sync::Arc;
use crate::utils::log;

use crate::ast::*;
use crate::ast::position::*;
use super::super::ast::parser::parse;

use crate::typecheck::*;

#[test]
#[wasm_bindgen_test]
fn typecheck_good() {
    let good_path = "./tiger_sources/good/";
    let source_files = read_dir(good_path).expect("read_dir");
    for direntry in source_files {
        let path = direntry.expect("direntry").path();
        let contents = read_to_string(&path).expect("read_to_string");
        let ast =  parse(&contents).expect("parser error");
        let type_env = initial_type_env();
        let value_env = initial_value_env();
        let res = type_exp(ast.clone() , &type_env, &value_env);
        match res {
            Ok(..) => (),
            Err(type_error) => panic!("Source: {:?}\n, AST: {:?}, Type Error: {:?}", &path, ast, type_error)
        }
    }
}

#[test]
#[wasm_bindgen_test]
fn bad_type() {
    let syntax_path = "./tiger_sources/type/";
    let source_files = read_dir(syntax_path).expect("read_dir");
    for direntry in source_files {
        let path = direntry.expect("direntry").path();
        let contents = read_to_string(&path).expect("read_to_string");
        let ast =  parse(&contents).expect("falla el parser");
        let type_env = initial_type_env();
        let value_env = initial_value_env();
        let typed = type_exp(ast.clone() , &type_env, &value_env);
        match typed {
            Err(..) => (),
            Ok(res) => panic!("Source: {:?}, AST: {:?}\n Type: {:?}", &path, ast, res.typ),
        }
    }
}

fn make_ast(exp: Exp) -> AST {
    AST {node: exp, pos: Pos {line: 0, column: 0}, typ: Arc::new(TigerType::Untyped)}
}

#[test]
#[wasm_bindgen_test]
fn unitexp() {
    let ast = make_ast(Exp::Unit);
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Ok(AST{typ, ..}) if *typ == TigerType::TUnit => (),
        Ok(AST{typ, ..}) => panic!("wrong type: {:?}", typ),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
#[wasm_bindgen_test]
fn nilexp() {
    let ast = make_ast(Exp::Nil);
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Ok(AST{typ, ..}) => match *typ {
            TigerType::TNil => (),
            _ => panic!("wrong type: {:?}", typ),
        },
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
#[wasm_bindgen_test]
fn breakexp() {
    let ast =  make_ast(Exp::Break);
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Ok(AST{typ, ..}) if *typ == TigerType::TUnit => (),
        Ok(AST{typ, ..}) => panic!("wrong type: {:?}", typ),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
#[wasm_bindgen_test]
fn intexp() {
    let ast =  make_ast(Exp::Int(1));
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Ok(AST{typ, ..}) if *typ == TigerType::TInt(R::RW) => (),
        Ok(AST{typ, ..}) => panic!("wrong type: {:?}", typ),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
#[wasm_bindgen_test]
fn stringexp() {
    let ast =  make_ast(Exp::String(String::from("lorem ipsum")));
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Ok(AST{typ, ..}) if *typ == TigerType::TString => (),
        Ok(AST{typ, ..}) => panic!("wrong type: {:?}", typ),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
#[wasm_bindgen_test]
fn varexp_simplevar_ok() {
    let ast =  make_ast(Exp::Var(make_var(VarKind::Simple(Symbol::from("foo")))));
    let type_env = initial_type_env();
    let mut value_env = initial_value_env();
    value_env.insert(Symbol::from("foo"), EnvEntry::Var{ty: Arc::new(TigerType::TInt(R::RW)),});
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Ok(AST{typ, ..}) if *typ == TigerType::TInt(R::RW) => (),
        Ok(AST{typ, ..}) => panic!("wrong type: {:?}", typ),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
#[wasm_bindgen_test]
fn varexp_simplevar_no_declarada() {
    let ast =  make_ast(Exp::Var(make_var(VarKind::Simple(Symbol::from("foo")))));
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Err(TypeError::UndeclaredSimpleVar(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(ast) => panic!("Should error, returns: {:?}", ast)
    }
}

#[test]
#[wasm_bindgen_test]
fn varexp_simplevar_no_es_simple() {
    let ast =  make_ast(Exp::Var(make_var(VarKind::Simple(Symbol::from("f")))));
    let type_env = initial_type_env();
    let mut value_env = initial_value_env();
    value_env.insert(Symbol::from("f"), EnvEntry::Func {
        formals: vec![],
        result: Arc::new(TigerType::TUnit),
    });
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Err(TypeError::NotSimpleVar(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
#[wasm_bindgen_test]
fn varexp_fieldvar_ok() {
    let ast = make_ast(Exp::Var(
        make_var(VarKind::Field(
            boxed_var(VarKind::Simple(Symbol::from("foo"))),
            Symbol::from("bar")))
    ));
    let mut type_env = initial_type_env();
    let mut value_env = initial_value_env();
    let field_type = Arc::new(TigerType::TInt(R::RW));
    let foo_type = Arc::new(TigerType::TRecord(
            vec![(String::from("bar"),
                field_type,
                0)], TypeId::new()));
    type_env.insert(Symbol::from("FooType"), foo_type.clone());
    value_env.insert(Symbol::from("foo"), EnvEntry::Var{
        ty: foo_type,
    });
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Ok(AST{typ, ..}) if *typ == TigerType::TInt(R::RW) => (),
        Ok(AST{typ, ..}) => panic!("wrong type: {:?}", typ),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
#[wasm_bindgen_test]
fn varexp_fieldvar_field_inexistente() {
    let ast = make_ast(Exp::Var(
        make_var(VarKind::Field(
            boxed_var(VarKind::Simple(Symbol::from("foo"))),
            Symbol::from("perro")))
    ));
    let mut type_env = initial_type_env();
    let mut value_env = initial_value_env();
    let foo_type = Arc::new(TigerType::TRecord(
            vec![(String::from("bar"),
                Arc::new(TigerType::TInt(R::RW)),
                0)], TypeId::new()));
    type_env.insert(Symbol::from("FooType"), foo_type.clone());
    value_env.insert(Symbol::from("foo"), EnvEntry::Var{
        ty: foo_type,
    });
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Err(TypeError::FieldDoesNotExist(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
#[wasm_bindgen_test]
fn varexp_fieldvar_sobre_tipo_no_record() {
    let ast = make_ast(Exp::Var(
        make_var(VarKind::Field(
            boxed_var(VarKind::Simple(Symbol::from("foo"))),
            Symbol::from("bar")))
    ));
    let mut type_env = initial_type_env();
    let mut value_env = initial_value_env();
    let foo_type = Arc::new(TigerType::TInt(R::RW));
    type_env.insert(Symbol::from("FooType"), foo_type.clone());
    value_env.insert(Symbol::from("foo"), EnvEntry::Var{
        ty: foo_type,
    });
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Err(TypeError::NotRecordType(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
#[wasm_bindgen_test]
fn varexp_subscriptvar_ok() {
    let ast = make_ast(Exp::Var(
        make_var(VarKind::Subscript(boxed_var(VarKind::Simple(Symbol::from("foo"))),
        boxed_ast(Exp::Int(0))),
    )));
    let mut type_env = initial_type_env();
    let mut value_env = initial_value_env();
    let foo_type = Arc::new(TigerType::TArray(
        Arc::new(TigerType::TInt(R::RW)),
        TypeId::new(),
    ));
    type_env.insert(Symbol::from("FooType"), foo_type.clone());
    value_env.insert(Symbol::from("foo"), EnvEntry::Var{
        ty: foo_type,
    });
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Ok(AST{typ, ..}) if *typ == TigerType::TInt(R::RW) => (),
        Ok(AST{typ, ..}) => panic!("wrong type: {:?}", typ),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
#[wasm_bindgen_test]
fn varexp_subscriptvar_indice_no_entero() {
    let ast = make_ast(Exp::Var(
        make_var(VarKind::Subscript(
            boxed_var(VarKind::Simple(Symbol::from("foo"))),
            boxed_ast(Exp::String(String::from("una string de indice :o"))),
        ))
    ));
    let mut type_env = initial_type_env();
    let mut value_env = initial_value_env();
    let foo_type = Arc::new(TigerType::TArray(
        Arc::new(TigerType::TInt(R::RW)),
        TypeId::new(),
    ));
    type_env.insert(Symbol::from("FooType"), foo_type.clone());
    value_env.insert(Symbol::from("foo"), EnvEntry::Var{
        ty: foo_type,
    });
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Err(TypeError::SubscriptNotInteger(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
#[wasm_bindgen_test]
fn varexp_subscriptvar_no_array() {
    let ast = make_ast(Exp::Var(
        make_var(VarKind::Subscript(boxed_var(VarKind::Simple(Symbol::from("foo"))),
        boxed_ast(Exp::Int(0))),
    )));
    let mut type_env = initial_type_env();
    let mut value_env = initial_value_env();
    let foo_type = Arc::new(TigerType::TInt(R::RW));
    type_env.insert(Symbol::from("FooType"), foo_type.clone());
    value_env.insert(Symbol::from("foo"), EnvEntry::Var{
        ty: foo_type,
    });
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Err(TypeError::NotArrayType(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
#[wasm_bindgen_test]
fn callexp_ok() {
    let ast = make_ast(Exp::Call {
        func: Symbol::from("f"),
        args: vec![],
    });
    let type_env = initial_type_env();
    let mut value_env = initial_value_env();
    value_env.insert(Symbol::from("f"), EnvEntry::Func {
        formals: vec![],
        result: Arc::new(TigerType::TUnit),
    });
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Ok(AST{typ, ..}) if *typ == TigerType::TUnit => (),
        Ok(AST{typ, ..}) => panic!("wrong type: {:?}", typ),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
#[wasm_bindgen_test]
fn callexp_args_de_mas() {
    let ast = make_ast(Exp::Call {
        func: Symbol::from("f"),
        args: vec![make_ast(Exp::Int(1))],
    });
    let type_env = initial_type_env();
    let mut value_env = initial_value_env();
    value_env.insert(Symbol::from("f"), EnvEntry::Func {
        formals: vec![],
        result: Arc::new(TigerType::TUnit),
    });
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Err(TypeError::TooManyArguments(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
#[wasm_bindgen_test]
fn callexp_args_de_menos() {
    let ast = make_ast(Exp::Call {
        func: Symbol::from("f"),
        args: vec![],
    });
    let type_env = initial_type_env();
    let mut value_env = initial_value_env();
    value_env.insert(Symbol::from("f"), EnvEntry::Func {
        formals: vec![Arc::new(TigerType::TInt(R::RW))],
        result: Arc::new(TigerType::TUnit),
    });
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Err(TypeError::TooFewArguments(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
#[wasm_bindgen_test]
fn callexp_funcion_no_declarada() {
    let ast = make_ast(Exp::Call {
        func: Symbol::from("f"),
        args: vec![],
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Err(TypeError::UndeclaredFunction(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
#[wasm_bindgen_test]
fn opexp_ok() {
    let ast = make_ast(Exp::Op {
        left: boxed_ast(Exp::Int(1)),
        oper: Oper::PlusOp,
        right: boxed_ast(Exp::Int(1)),
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Ok(AST{typ, ..}) if *typ == TigerType::TInt(R::RW) => (),
        Ok(AST{typ, ..}) => panic!("wrong type: {:?}", typ),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
#[wasm_bindgen_test]
fn opexp_tipos_distintos() {
    let ast = make_ast(Exp::Op {
        left: boxed_ast(Exp::Int(1)),
        oper: Oper::PlusOp,
        right: boxed_ast(Exp::String(String::from("perro"))),
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Err(TypeError::TypeMismatch(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
#[wasm_bindgen_test]
fn recordexp_ok() {
    let ast = make_ast(Exp::Record {
        fields: vec![(Symbol::from("baz"), boxed_ast(Exp::Int(1)))],
        typ: Symbol::from("FooType"),
    });
    let mut type_env = initial_type_env();
    let value_env = initial_value_env();
    let field_type = Arc::new(TigerType::TInt(R::RW));
    let foo_type = Arc::new(TigerType::TRecord(
            vec![(String::from("baz"),
                field_type,
                0)], TypeId::new()));
    type_env.insert(Symbol::from("FooType"), foo_type.clone());
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Ok(AST{typ, ..}) if *typ == *foo_type => (),
        Ok(AST{typ, ..}) => panic!("wrong type: {:?}", typ),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
#[wasm_bindgen_test]
fn recordexp_tipo_inexistente() {
    let ast = make_ast(Exp::Record {
        fields: vec![(Symbol::from("baz"), boxed_ast(Exp::Int(1)))],
        typ: Symbol::from("FooType"),
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Err(TypeError::UndeclaredType(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
#[wasm_bindgen_test]
fn recordexp_con_tipo_no_record() {
    let ast = make_ast(Exp::Record {
        fields: vec![(Symbol::from("baz"), boxed_ast(Exp::Int(1)))],
        typ: Symbol::from("FooType"),
    });
    let mut type_env = initial_type_env();
    let value_env = initial_value_env();
    type_env.insert(Symbol::from("FooType"), Arc::new(TigerType::TInt(R::RW)));
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Err(TypeError::NotRecordType(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
#[wasm_bindgen_test]
fn arrayexp_ok() {
    let ast = make_ast(Exp::Array {
        typ: Symbol::from("FooType"),
        size: boxed_ast(Exp::Int(1)),
        init: boxed_ast(Exp::Int(2))
    });
    let mut type_env = initial_type_env();
    let value_env = initial_value_env();
    let foo_type = Arc::new(TigerType::TArray(
        Arc::new(TigerType::TInt(R::RW)),
        TypeId::new(),
    ));
    type_env.insert(Symbol::from("FooType"), foo_type.clone());
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Ok(AST{typ, ..}) if *typ == *foo_type => (),
        Ok(AST{typ, ..}) => panic!("wrong type: {:?}", typ),
        Err(..) => panic!("array")
    }
}

#[test]
#[wasm_bindgen_test]
fn arrayexp_size_no_int() {
    let ast = make_ast(Exp::Array {
        typ: Symbol::from("FooType"),
        size: boxed_ast(Exp::String(String::from("perro"))),
        init: boxed_ast(Exp::Int(2))
    });
    let mut type_env = initial_type_env();
    let value_env = initial_value_env();
    let foo_type = Arc::new(TigerType::TArray(
        Arc::new(TigerType::TInt(R::RW)),
        TypeId::new(),
    ));
    type_env.insert(Symbol::from("FooType"), foo_type);
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Err(TypeError::NonIntegerSize(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
#[wasm_bindgen_test]
fn arrayexp_tipos_distintos() {
    let ast = make_ast(Exp::Array {
        typ: Symbol::from("FooType"),
        size: boxed_ast(Exp::Int(1)),
        init: boxed_ast(Exp::String(String::from("perro")))
    });
    let mut type_env = initial_type_env();
    let value_env = initial_value_env();
    let foo_type = Arc::new(TigerType::TArray(
        Arc::new(TigerType::TInt(R::RW)),
        TypeId::new(),
    ));
    type_env.insert(Symbol::from("FooType"), foo_type);
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Err(TypeError::TypeMismatch(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
#[wasm_bindgen_test]
fn arrayexp_tipo_no_array() {
    let ast = make_ast(Exp::Array {
        typ: Symbol::from("FooType"),
        size: boxed_ast(Exp::Int(1)),
        init: boxed_ast(Exp::String(String::from("perro")))
    });
    let mut type_env = initial_type_env();
    let value_env = initial_value_env();
    let foo_type = Arc::new(TigerType::TInt(R::RW));
    type_env.insert(Symbol::from("FooType"), foo_type);
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Err(TypeError::NotArrayType(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
#[wasm_bindgen_test]
fn arrayexp_tipo_no_existe() {
    let ast = make_ast(Exp::Array {
        typ: Symbol::from("FooType"),
        size: boxed_ast(Exp::Int(1)),
        init: boxed_ast(Exp::String(String::from("perro")))
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Err(TypeError::UndeclaredType(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
#[wasm_bindgen_test]
fn seqexp_ok() {
    let ast = make_ast(Exp::Seq(vec![
        make_ast(Exp::Int(1)),
        make_ast(Exp::Int(1)),
    ]));
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Ok(AST{typ, ..}) if *typ == TigerType::TInt(R::RW) => (),
        Ok(AST{typ, ..}) => panic!("wrong type: {:?}", typ),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}
// Se puede testear algo mas de Exp::Seq? Hay alguna condicion del ultimo tipo?

#[test]
#[wasm_bindgen_test]
fn assignexp_ok() {
    let ast = make_ast(Exp::Assign{
        var: make_var(VarKind::Simple(Symbol::from("foo"))),
        exp: boxed_ast(Exp::Int(1)),
    });
    let type_env = initial_type_env();
    let mut value_env = initial_value_env();
    let env_entry = EnvEntry::Var{
        ty: Arc::new(TigerType::TInt(R::RW)),
    };
    value_env.insert(Symbol::from("foo"), env_entry);
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Ok(AST{typ, ..}) if *typ == TigerType::TUnit => (),
        Ok(AST{typ, ..}) => panic!("wrong type: {:?}", typ),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
#[wasm_bindgen_test]
fn assignexp_variable_no_existe() {
    let ast = make_ast(Exp::Assign{
        var: make_var(VarKind::Simple(Symbol::from("foo"))),
        exp: boxed_ast(Exp::Int(1)),
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Err(TypeError::UndeclaredSimpleVar(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
#[wasm_bindgen_test]
fn assignexp_tipos_distintos() {
    let ast = make_ast(Exp::Assign{
        var: make_var(VarKind::Simple(Symbol::from("foo"))),
        exp: boxed_ast(Exp::String(String::from("perro"))),
    });
    let type_env = initial_type_env();
    let mut value_env = initial_value_env();
    let env_entry = EnvEntry::Var{
        ty: Arc::new(TigerType::TInt(R::RW)),
    };
    value_env.insert(Symbol::from("foo"), env_entry);
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Err(TypeError::TypeMismatch(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
#[wasm_bindgen_test]
fn assignexp_variable_read_only() {
    let ast = make_ast(Exp::Assign{
        var: make_var(VarKind::Simple(Symbol::from("i"))),
        exp: boxed_ast(Exp::Int(2)),
    });
    let type_env = initial_type_env();
    let mut value_env = initial_value_env();
    let env_entry = EnvEntry::Var{
        ty: Arc::new(TigerType::TInt(R::RO)),
    };
    value_env.insert(Symbol::from("i"), env_entry);
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Err(TypeError::ReadOnlyAssignment(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
#[wasm_bindgen_test]
fn ifexp_ok() {
    let ast = make_ast(Exp::If {
        test: boxed_ast(Exp::Int(0)),
        then_: boxed_ast(Exp::Int(1)),
        else_: Some(boxed_ast(Exp::Int(2)))
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Ok(AST{typ, ..}) if *typ == TigerType::TInt(R::RW) => (),
        Ok(AST{typ, ..}) => panic!("wrong type: {:?}", typ),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
#[wasm_bindgen_test]
fn ifexp_test_no_entero() {
    let ast = make_ast(Exp::If {
        test: boxed_ast(Exp::String(String::from("perro"))),
        then_: boxed_ast(Exp::Int(1)),
        else_: Some(boxed_ast(Exp::Int(2)))
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Err(TypeError::NonIntegerCondition(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
#[wasm_bindgen_test]
fn ifexp_tipos_then_else_distintos() {
    let ast = make_ast(Exp::If {
        test: boxed_ast(Exp::Int(0)),
        then_: boxed_ast(Exp::Int(1)),
        else_: Some(boxed_ast(Exp::String(String::from("perro")))),
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Err(TypeError::ThenElseTypeMismatch(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
#[wasm_bindgen_test]
fn ifexp_sin_else_no_unit() {
    let ast = make_ast(Exp::If {
        test: boxed_ast(Exp::Int(0)),
        then_: boxed_ast(Exp::Int(1)),
        else_: None
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Err(TypeError::NonUnitBody(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
#[wasm_bindgen_test]
fn whileexp_ok() {
    let ast = make_ast(Exp::While {
        test: boxed_ast(Exp::Int(0)),
        body: boxed_ast(Exp::Unit),
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Ok(AST{typ, ..}) if *typ == TigerType::TUnit => (),
        Ok(AST{typ, ..}) => panic!("wrong type: {:?}", typ),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
#[wasm_bindgen_test]
fn whileexp_condicion_no_entera() {
    let ast = make_ast(Exp::While {
        test: boxed_ast(Exp::Unit),
        body: boxed_ast(Exp::Unit),
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Err(TypeError::NonIntegerCondition(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
#[wasm_bindgen_test]
fn forexp_ok() {
    let ast = make_ast(Exp::For {
        var: Symbol::from("i"),
        escape: false,
        lo: boxed_ast(Exp::Int(1)),
        hi: boxed_ast(Exp::Int(10)),
        body: boxed_ast(Exp::Unit),
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Ok(AST{typ, ..}) if *typ == TigerType::TUnit => (),
        Ok(AST{typ, ..}) => panic!("wrong type: {:?}", typ),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
#[wasm_bindgen_test]
fn forexp_iterador_es_usable() {
    let ast = make_ast(Exp::For {
        var: Symbol::from("i"),
        escape: false,
        lo: boxed_ast(Exp::Int(1)),
        hi: boxed_ast(Exp::Int(10)),
        body: boxed_ast(Exp::Seq(vec![
            make_ast(Exp::Var(make_var(VarKind::Simple(Symbol::from("i"))))),
            make_ast(Exp::Unit)])),
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Ok(AST{typ, ..}) if *typ == TigerType::TUnit => (),
        Ok(AST{typ, ..}) => panic!("wrong type: {:?}", typ),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
#[wasm_bindgen_test]
fn forexp_body_no_es_unit() {
    let ast = make_ast(Exp::For {
        var: Symbol::from("i"),
        escape: false,
        lo: boxed_ast(Exp::Int(1)),
        hi: boxed_ast(Exp::Int(10)),
        body: boxed_ast(Exp::Int(2)),
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Err(TypeError::NonUnitBody(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
#[wasm_bindgen_test]
fn forexp_lo_no_es_int() {
    let ast = make_ast(Exp::For {
        var: Symbol::from("i"),
        escape: false,
        lo: boxed_ast(Exp::Unit),
        hi: boxed_ast(Exp::Int(10)),
        body: boxed_ast(Exp::Unit),
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Err(TypeError::NonIntegerForRange(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
#[wasm_bindgen_test]
fn forexp_hi_no_es_int() {
    let ast = make_ast(Exp::For {
        var: Symbol::from("i"),
        escape: false,
        lo: boxed_ast(Exp::Int(1)),
        hi: boxed_ast(Exp::Unit),
        body: boxed_ast(Exp::Unit),
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Err(TypeError::NonIntegerForRange(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
#[wasm_bindgen_test]
fn letexp_vardec_sin_tipo_ok() {
    let ast =  make_ast(Exp::Let {
        decs: vec![Dec::VarDec(
            _VarDec::new(
                Symbol::from("foo"),
                None,
                boxed_ast(Exp::Int(4))
            ),
            Pos{line: 0, column: 0}
        )],
        body: boxed_ast(Exp::Var(make_var(VarKind::Simple(Symbol::from("foo")))))
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Ok(AST{typ, ..}) if *typ == TigerType::TInt(R::RW) => (),
        Ok(AST{typ, ..}) => panic!("wrong type: {:?}", typ),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
#[wasm_bindgen_test]
fn letexp_vardec_con_tipo_ok() {
    let ast =  make_ast(Exp::Let {
        decs: vec![Dec::VarDec(
            _VarDec::new(
                Symbol::from("foo"),
                Some(Symbol::from("int")),
                boxed_ast(Exp::Int(4)),
            ),
            Pos{line: 0, column: 0}
        )],
        body: boxed_ast(Exp::Var(make_var(VarKind::Simple(Symbol::from("foo")))))
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();

    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Ok(AST{typ, ..}) if *typ == TigerType::TInt(R::RW) => (),
        Ok(AST{typ, ..}) => panic!("wrong type: {:?}", typ),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
#[wasm_bindgen_test]
fn letexp_vardec_tipo_no_esta_declarado() {
    let ast =  make_ast(Exp::Let {
        decs: vec![Dec::VarDec(
            _VarDec::new(
                Symbol::from("foo"),
                Some(Symbol::from("un_tipo_no_declarado")),
                boxed_ast(Exp::Int(4)),
            ),
            Pos{line: 0, column: 0}
        )],
        body: boxed_ast(Exp::Unit)
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Err(TypeError::UndeclaredType(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
#[wasm_bindgen_test]
fn letexp_vardec_tipos_distintos() {
    let ast =  make_ast(Exp::Let {
        decs: vec![Dec::VarDec(
            _VarDec::new(
                Symbol::from("foo"),
                Some(Symbol::from("string")),
                boxed_ast(Exp::Int(4))
            ),
            Pos{line: 0, column: 0}
        )],
        body: boxed_ast(Exp::Unit)
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Err(TypeError::TypeMismatch(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
#[wasm_bindgen_test]
fn letexp_typedec_name_ok() {
    let ast =  make_ast(Exp::Let {
        decs: vec![
            Dec::TypeDec(vec![(
                _TypeDec::new(
                    Symbol::from("FooType"),
                    Ty::Name(Symbol::from("int"))
                ),
                Pos{line: 0, column: 0}
            )]),
            Dec::VarDec(
                _VarDec::new(
                    Symbol::from("foo"),
                    Some(Symbol::from("FooType")),
                    boxed_ast(Exp::Int(4))
                ),
                Pos{line: 0, column: 0}
            ),
        ],
        body: boxed_ast(Exp::Var(make_var(VarKind::Simple(Symbol::from("foo")))))
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Ok(AST{typ, ..}) if *typ == TigerType::TInt(R::RW) => (),
        Ok(AST{typ, ..}) => panic!("wrong type: {:?}", typ),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
#[wasm_bindgen_test]
fn letexp_typedec_array_ok() {
    let ast =  make_ast(Exp::Let {
        decs: vec![
            Dec::TypeDec(vec![(
                _TypeDec::new(
                    Symbol::from("FooType"),
                    Ty::Array(Symbol::from("int"))
                ),
                Pos{line: 0, column: 0}
            )]),
            Dec::VarDec(
                _VarDec::new(
                    Symbol::from("foo"),
                    Some(Symbol::from("FooType")),
                    boxed_ast(Exp::Array {
                        typ: Symbol::from("FooType"),
                        size:boxed_ast(Exp::Int(1)),
                        init: boxed_ast(Exp::Int(2)),
                    })
                ),
                Pos{line: 0, column: 0}
            ),
        ],
        body: boxed_ast(Exp::Var(
            make_var(VarKind::Subscript(
                boxed_var(VarKind::Simple(Symbol::from("foo"))),
                boxed_ast(Exp::Int(0))
            ))
        ))
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Ok(AST{typ, ..}) if *typ == TigerType::TInt(R::RW) => (),
        Ok(AST{typ, ..}) => panic!("wrong type: {:?}", typ),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
#[wasm_bindgen_test]
fn letexp_typedec_record_ok() {
    let ast =  make_ast(Exp::Let {
        decs: vec![
            Dec::TypeDec(vec![(
                _TypeDec::new(
                    Symbol::from("FooType"),
                    Ty::Record(vec![
                        Field {
                            name: Symbol::from("bar"),
                            typ: Ty::Name(Symbol::from("int")),
                            escape: false,
                        }
                    ])
                ),
                Pos{line: 0, column: 1}
            )]),
            Dec::VarDec(
                _VarDec::new(
                    Symbol::from("foo"),
                    Some(Symbol::from("FooType")),
                    boxed_ast(Exp::Record {
                        fields: vec![(Symbol::from("bar"), boxed_ast(Exp::Int(1)))],
                        typ: Symbol::from("FooType"),
                    })
                ),
                Pos{line: 0, column: 2}
            )],
        body: boxed_ast(Exp::Var(
            make_var(VarKind::Field(
                boxed_var(VarKind::Simple(Symbol::from("foo"))),
                Symbol::from("bar")
            ))
        ))
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Ok(AST{typ, ..}) if *typ == TigerType::TInt(R::RW) => (),
        Ok(AST{typ, ..}) => panic!("wrong type: {:?}", typ),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
#[wasm_bindgen_test]
fn letexp_typedec_recursion_infinita() {
   let ast =  make_ast(Exp::Let {
        decs: vec![Dec::TypeDec(vec![
            (_TypeDec::new(Symbol::from("FooType"), Ty::Name(Symbol::from("BaazType"))), Pos{line: 0, column: 0}),
            (_TypeDec::new(Symbol::from("BaazType"), Ty::Name(Symbol::from("FooType"))), Pos{line: 0, column: 0}),
        ])],
        body: boxed_ast(Exp::Unit)
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Err(TypeError::TypeCycle(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}
#[test]
#[wasm_bindgen_test]
fn test_recursive_ok() {
   let ast =  make_ast(Exp::Let {
        decs: vec![Dec::TypeDec(vec![
            (_TypeDec::new(Symbol::from("C"), Ty::Name(Symbol::from("B"))), Pos{line: 0, column: 0}),
            (_TypeDec::new(Symbol::from("B"), Ty::Name(Symbol::from("A"))), Pos{line: 0, column: 0}),
            (_TypeDec::new(Symbol::from("A"), Ty::Name(Symbol::from("int"))), Pos{line: 0, column: 0}),
        ])],
        body: boxed_ast(Exp::Unit)
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Ok(AST{typ, ..}) if *typ == TigerType::TUnit => (),
        Ok(AST{typ, ..}) => panic!("wrong type: {:?}", typ),
        Err(..) => panic!("type error"),
    }
}

#[test]
#[wasm_bindgen_test]
fn letexp_typedec_referencia_tipo_inexistente() {
    let ast =  make_ast(Exp::Let {
        decs: vec![Dec::TypeDec(vec![(
            _TypeDec::new(
                Symbol::from("FooType"),
                Ty::Name(Symbol::from("BaazType"))
            ),
            Pos{line: 0, column: 0}
        )])],
        body: boxed_ast(Exp::Unit)
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Err(TypeError::UndeclaredType(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
#[wasm_bindgen_test]
fn record_type_cycle_ok() {
    let ast =  make_ast(Exp::Let {
        decs: vec![
            Dec::TypeDec(vec![(
                _TypeDec::new(
                    Symbol::from("List"),
                    Ty::Record(vec![
                        Field {
                            name: Symbol::from("head"),
                            typ: Ty::Name(Symbol::from("int")),
                            escape: false,
                        },
                        Field {
                            name: Symbol::from("tail"),
                            typ: Ty::Name(Symbol::from("List")),
                            escape: false,
                        }
                    ])
                ),
                Pos{line: 0, column: 1}
            )]),
            Dec::VarDec(
                _VarDec::new(
                    Symbol::from("foo"),
                    Some(Symbol::from("List")),
                    boxed_ast(Exp::Record {
                        fields: vec![
                            (Symbol::from("head"), boxed_ast(Exp::Int(1))),
                            (Symbol::from("tail"), boxed_ast(Exp::Record {
                                fields: vec![
                                    (Symbol::from("head"), boxed_ast(Exp::Int(2))),
                                    (Symbol::from("tail"), boxed_ast(Exp::Record {
                                        fields: vec![
                                            (Symbol::from("head"), boxed_ast(Exp::Int(3))),
                                            (Symbol::from("tail"), boxed_ast(Exp::Record {
                                                fields: vec![
                                                    (Symbol::from("head"), boxed_ast(Exp::Int(4))),
                                                    (Symbol::from("tail"), boxed_ast(Exp::Nil))
                                                ],
                                                typ: Symbol::from("List"),
                                            }))
                                        ],
                                        typ: Symbol::from("List"),
                                    }))
                                ],
                                typ: Symbol::from("List"),
                            }))
                        ],
                        typ: Symbol::from("List"),
                    })
                ),
                Pos{line: 0, column: 2}
            )],
        body: boxed_ast(Exp::Var(
            make_var(VarKind::Field(
                boxed_var(VarKind::Simple(Symbol::from("foo"))),
                Symbol::from("head")
            ))
        ))
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Ok(AST{typ, ..}) if *typ == TigerType::TInt(R::RW) => (),
        Ok(AST{typ, ..}) => panic!("wrong type: {:?}", typ),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
#[wasm_bindgen_test]
fn letexp_functiondec_ok() {
    let ast =  make_ast(Exp::Let {
        decs: vec![Dec::FunctionDec(vec![(
            _FunctionDec::new(
                Symbol::from("foo"),
                vec![Field {
                    name: Symbol::from("arg"),
                    typ: Ty::Name(Symbol::from("int")),
                    escape: false,
                }],
                None,
                boxed_ast(Exp::Unit)
            ),
            Pos{line: 0, column: 0}
        )])],
        body: boxed_ast(Exp::Unit)
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Ok(AST{typ, ..}) if *typ == TigerType::TUnit => (),
        Ok(AST{typ, ..}) => panic!("wrong type: {:?}", typ),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
#[wasm_bindgen_test]
fn letexp_functiondec_llamada_en_bloque_ok() {
    let ast =  make_ast(Exp::Let {
        decs: vec![
            Dec::FunctionDec(vec![(
                _FunctionDec::new(
                    Symbol::from("foo"),
                    vec![Field {
                        name: Symbol::from("arg1"),
                        typ: Ty::Name(Symbol::from("int")),
                        escape: false,
                    }],
                    Some(Symbol::from("int")),
                    boxed_ast(Exp::Var(make_var(VarKind::Simple(Symbol::from("arg1"))))),
                ),
                Pos{line: 0, column: 0}
            )]),
            Dec::FunctionDec(vec![(
                _FunctionDec::new(
                    Symbol::from("baaz"),
                    vec![Field {
                        name: Symbol::from("arg2"),
                        typ: Ty::Name(Symbol::from("int")),
                        escape: false,
                    }],
                    Some(Symbol::from("int")),
                    boxed_ast(Exp::Call {
                        func: Symbol::from("foo"),
                        args: vec![make_ast(Exp::Var(make_var(VarKind::Simple(Symbol::from("arg2")))))],
                    })
                ),
                Pos{line: 0, column: 0}
            )]),
        ],
        body: boxed_ast(Exp::Call {
            func: Symbol::from("baaz"),
            args: vec![make_ast(Exp::Int(2))]
        })
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Ok(AST{typ, ..}) if *typ == TigerType::TInt(R::RW) => (),
        Ok(AST{typ, ..}) => panic!("wrong type: {:?}", typ),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
#[wasm_bindgen_test]
fn letexp_functiondec_body_no_tipa() {
    let ast =  make_ast(Exp::Let {
        decs: vec![Dec::FunctionDec(vec![(
            _FunctionDec::new(
                Symbol::from("foo"),
                vec![Field {
                    name: Symbol::from("arg"),
                    typ: Ty::Name(Symbol::from("int")),
                    escape: false,
                }],
                None,
                boxed_ast(Exp::Var(make_var(VarKind::Simple(Symbol::from("baaz"))))), // undeclared
            ),
            Pos{line: 0, column: 0}
        )])],
        body: boxed_ast(Exp::Unit)
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Err(TypeError::UndeclaredSimpleVar(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
#[wasm_bindgen_test]
fn letexp_functiondec_body_distinto_result() {
    let ast =  make_ast(Exp::Let {
        decs: vec![Dec::FunctionDec(vec![(
            _FunctionDec::new(
                Symbol::from("foo"),
                vec![Field {
                    name: Symbol::from("arg"),
                    typ: Ty::Name(Symbol::from("int")),
                    escape: false,
                }],
                None,
                boxed_ast(Exp::Int(2)),
            ),
            Pos{line: 0, column: 0}
        )])],
        body: boxed_ast(Exp::Unit)
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Err(TypeError::TypeMismatch(_)) => (),
        Err(type_error) => panic!("Wrong type error: {:?}", type_error),
        Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
    }
}

#[test]
#[wasm_bindgen_test]
fn letexp_functiondec_params_repetidos() {
    let ast =  make_ast(Exp::Let {
        decs: vec![Dec::FunctionDec(vec![(
            _FunctionDec::new(
                Symbol::from("foo"),
                vec![Field {
                    name: Symbol::from("arg"),
                    typ: Ty::Name(Symbol::from("int")),
                    escape: false,
                }],
                None,
                boxed_ast(Exp::Unit)
            ),
            Pos{line: 0, column: 0}
        )])],
        body: boxed_ast(Exp::Unit)
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Ok(AST{typ, ..}) if *typ == TigerType::TUnit => (),
        Ok(AST{typ, ..}) => panic!("wrong type: {:?}", typ),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
#[wasm_bindgen_test]
fn letexp_functiondec_nombres_repetidos() {
    let ast =  make_ast(Exp::Let {
        decs: vec![Dec::FunctionDec(vec![(
            _FunctionDec::new(
                Symbol::from("foo"),
                vec![Field {
                    name: Symbol::from("arg"),
                    typ: Ty::Name(Symbol::from("int")),
                    escape: false,
                }],
                None,
                boxed_ast(Exp::Unit)
            ),
            Pos{line: 0, column: 0}
        )])],
        body: boxed_ast(Exp::Unit)
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Ok(AST{typ, ..}) if *typ == TigerType::TUnit => (),
        Ok(AST{typ, ..}) => panic!("wrong type: {:?}", typ),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
#[wasm_bindgen_test]
fn letexp_functiondec_recursivas() {
    let ast =  make_ast(Exp::Let {
        decs: vec![Dec::FunctionDec(vec![(
            _FunctionDec::new(
                Symbol::from("foo"),
                vec![Field {
                    name: Symbol::from("arg"),
                    typ: Ty::Name(Symbol::from("int")),
                    escape: false,
                }],
                None,
                boxed_ast(Exp::Unit)
            ),
            Pos{line: 0, column: 0})])],
        body: boxed_ast(Exp::Unit)
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Ok(AST{typ, ..}) if *typ == TigerType::TUnit => (),
        Ok(AST{typ, ..}) => panic!("wrong type: {:?}", typ),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}

#[test]
#[wasm_bindgen_test]
fn letexp_todas_las_decs_ok() {
    let ast =  make_ast(Exp::Let {
        decs: vec![
            Dec::TypeDec(vec![(
                _TypeDec::new(
                    Symbol::from("FooType"),
                    Ty::Name(Symbol::from("int")),
                ),
                Pos{line: 0, column: 0}
            )]),
            Dec::VarDec(
                _VarDec::new(
                    Symbol::from("foo"),
                    Some(Symbol::from("FooType")),
                    boxed_ast(Exp::Int(4))
                ),
                Pos{line: 0, column: 0}
            ),
            Dec::FunctionDec(vec![(
                _FunctionDec::new(
                    Symbol::from("baaz"),
                    vec![Field {
                        name: Symbol::from("bar"),
                        typ: Ty::Name(Symbol::from("FooType")),
                        escape: false,
                    }],
                    Some(Symbol::from("FooType")),
                    boxed_ast(Exp::Var(make_var(VarKind::Simple(Symbol::from("bar")))))
                ),
                Pos{line: 0, column: 0}
            )]),
        ],
        body: boxed_ast(Exp::Call {
            func: Symbol::from("baaz"),
            args: vec![make_ast(Exp::Var(make_var(VarKind::Simple(Symbol::from("foo")))))]
        })
    });
    let type_env = initial_type_env();
    let value_env = initial_value_env();
    let res = type_exp(ast, &type_env, &value_env);
    match res {
        Ok(AST{typ, ..}) if *typ == TigerType::TInt(R::RW) => (),
        Ok(AST{typ, ..}) => panic!("wrong type: {:?}", typ),
        Err(type_error) => panic!("type error: {:?}", type_error)
    }
}