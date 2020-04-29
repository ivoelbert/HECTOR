mod munch;
mod enviroment;
use munch::munch_stm;
use enviroment::{LocalEnv, FunctionEnv};
extern crate parity_wasm;
extern crate wasmprinter;

use parity_wasm::builder::ModuleBuilder;
use parity_wasm::{builder, elements::*, elements::Instruction::*};
pub use parity_wasm::SerializationError;

use crate::canonization::CanonFrag;
use crate::tree::Tree;
use crate::tree::frame::*;
use crate::tree::level::Label;

pub type Wasm = Module;

pub fn emit(frags: Vec<CanonFrag>) -> (String, Vec<u8>) {
	// let module = builder::module().build();
	let (module_builder, functions) = frags.into_iter().fold(
        (builder::module(), FunctionEnv::new()),
        |(module, fenv): (ModuleBuilder, FunctionEnv), frag: CanonFrag| -> (ModuleBuilder, FunctionEnv) {
            match frag {
                CanonFrag::ConstString(label, string) => (emit_string_global(label, string, module), fenv),
                CanonFrag::Proc{body, frame}    => emit_function(body, frame, module, fenv),
            }
        }
	);
	let module = module_builder
		// STACK POINTER
		.global()
			.value_type().i32()
			.init_expr(I32Const(0))
			.mutable()
			.build()
		// FRAME POINTER
		.global()
			.value_type().i32()
			.init_expr(I32Const(0))
			.mutable()
			.build()
		// RETURN VALUE
		.global()
			.value_type().i32()
			.init_expr(I32Const(0))
			.mutable()
			.build()
		.memory()
			// CHEQUEAR: esto alcanza o hay que meter un grow?
			.with_min(1024)
			.build()
		.function()
			.signature()
				.with_return_type(Some(ValueType::I32))
				.build()
			.body()
				.with_locals(vec![])
				.with_instructions(Instructions::new(vec![I32Const(0), Call(functions.get_last_index()), End]))
				.build()
			.build()
		.with_export(builder::export()
			.field("return")
			// .internal().func(1)
			.internal().global(2)
			.build()
		)
		.build();
	// println!("{:#?}", &module);
	// (format!("{:?}", &module), parity_wasm::serialize(module).unwrap())
	// parity_wasm::serialize_to_file("asd.wasm", module.clone()).unwrap();
	let bytes = parity_wasm::serialize(module).unwrap();
	let text = wasmprinter::print_bytes(&bytes).unwrap();
	console_log!("{:?}", &text);
	(text, bytes)
}

fn emit_string_global(label: Label, string: String, module: ModuleBuilder) -> ModuleBuilder {
	module
	// TODO: string deberian ir en el segmento de datos de alguna forma que no entiendo bien
}

fn emit_function(tree_body: Vec<Tree::Stm>, frame: Frame, module: ModuleBuilder, mut function_env: FunctionEnv) -> (ModuleBuilder, FunctionEnv) {
	function_env.insert(frame.label.clone());
	let (locals, params) = LocalEnv::from_frame(&frame);
	let (instructions, locals) : (Vec<Instruction>, LocalEnv) = tree_body
	.into_iter()
	.fold((vec![], locals), |(mut instructions, locals): (Vec<Instruction>, LocalEnv), stm: Tree::Stm| -> (Vec<Instruction>, LocalEnv) {
		let (mut ins, locals) = munch_stm(stm, locals, &function_env);
		instructions.append(&mut ins);
		(instructions, locals)
	});
	(module.function()
		.signature()
			.with_params(params)
			.with_return_type(Some(ValueType::I32))
			.build()
		.body()
			.with_locals(locals.finish())
			.with_instructions(Instructions::new(instructions))
			.build()
		.build(), function_env)
}