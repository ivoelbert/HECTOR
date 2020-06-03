use std::convert::{TryFrom, TryInto};

mod munch;
mod enviroment;
use munch::munch_body;
use enviroment::{LocalEnv, FunctionEnv, StringEnv, LabelEnv, initial_function_env};
extern crate parity_wasm;
extern crate wasmprinter;

use parity_wasm::{builder::{self, ModuleBuilder}, elements::*, elements::Instruction::*};
pub use parity_wasm::SerializationError;

use crate::canonization::{CanonFrag, Block};
use crate::tree::Tree;
use crate::tree::frame::*;
use crate::tree::level::Label;

#[allow(clippy::type_complexity)]
fn destructure_frags(frags: Vec<CanonFrag>) -> (Vec<(Label, String)>, Vec<(Vec<Block>, Frame)>) {
	let mut procs = vec![];
	let mut strings = vec![];
	for frag in frags.into_iter() {
		match frag {
			CanonFrag::ConstString(label, string) => strings.push((label, string)),
			CanonFrag::Proc{body, frame}    => procs.push((body, frame)),
		}
	};
	(strings, procs)
}

pub fn emit_module(frags: Vec<CanonFrag>) -> (String, Vec<u8>) {
	// let module = builder::module().build();
	let (strings, procs) = destructure_frags(frags);
	let mut function_env = initial_function_env();
	let current_index = function_env.len();
	function_env.extend(
		procs
		.iter()
		.enumerate()
		.map(|(i, (_, frame))| (frame.label.clone(), (current_index + i).try_into().unwrap()))
	);
	let string_env = strings
		.iter()
		.fold(StringEnv::new(), |string_env, (label, string)| string_env.insert(label.clone(), &string));
	let module = builder::module();
	let module = emit_imports(module);
	let module = strings
		.into_iter()
		// Data Section
		.fold(module, |module, (_, string)| emit_string(string, module));
	let module = procs
		.into_iter()
		// Function, Signature Sections
		.fold(module, |module, (blocks, frame)| emit_function(blocks, frame, &function_env, &string_env, module))
		// Main Wrapper
		.function()
			// .main()
			.signature()
				.with_return_type(Some(ValueType::I32))
				// .with_return_type(None)
				.build()
			.body()
				.with_locals(vec![])
				.with_instructions(Instructions::new(vec![
					I32Const(0), // tigermain's static link
					Call((function_env.len() - 1).try_into().unwrap()),
					End]))
				.build()
			.build()
		// Stack Pointer
		.global()
			.value_type().i32()
			.init_expr(I32Const(0))
			.mutable()
			.build()
		// Frame Pointer
		.global()
			.value_type().i32()
			.init_expr(I32Const(0))
			.mutable()
			.build()
		// Return Value
		.global()
			.value_type().i32()
			.init_expr(I32Const(0))
			.mutable()
			.build()
		// Next Jump
		.global()
			.value_type().i32()
			.init_expr(I32Const(0))
			.mutable()
			.build()
		.memory()
			// CHEQUEAR: esto alcanza o hay que meter un grow?
			.with_min(1024)
			.build()
		// // Return value export
		// .with_export(builder::export()
		// 	.field("return")
		// 	.internal().global(2)
		// 	.build()
		// )
		// Main Wrapper value export
		.with_export(builder::export()
			.field("tigermain_wrapper")
			.internal().func(function_env.len().try_into().unwrap())
			.build()
		)
		.build();
	// println!("{:#?}", &module);
	// (format!("{:?}", &module), parity_wasm::serialize(module).unwrap())
	// parity_wasm::serialize_to_file("asd.wasm", module.clone()).unwrap();
	let bytes = parity_wasm::serialize(module).unwrap();
	let text = wasmprinter::print_bytes(&bytes).unwrap();
	// console_log!("{:?}", &text);
	(text, bytes)
}

fn emit_imports(module: ModuleBuilder) -> ModuleBuilder {

	type External = (&'static str, Vec<ValueType>, Option<ValueType>);
	let standard_library : Vec<External> = vec![
		("print", vec![ValueType::I32], None),
		("flush", vec![], None),
		("getchar", vec![ValueType::I32], Some(ValueType::I32)),
		("ord", vec![ValueType::I32], Some(ValueType::I32)),
		("chr", vec![ValueType::I32], Some(ValueType::I32)),
		("size", vec![ValueType::I32], Some(ValueType::I32)),
		("substring", vec![ValueType::I32], Some(ValueType::I32)),
		("concat", vec![ValueType::I32, ValueType::I32], Some(ValueType::I32)),
		("not", vec![ValueType::I32], Some(ValueType::I32)),
		("exit", vec![ValueType::I32], None),
		("alloc_array", vec![ValueType::I32, ValueType::I32], Some(ValueType::I32)),
        ("alloc_record", vec![ValueType::I32, ValueType::I32], Some(ValueType::I32)),
        ("check_index_array", vec![ValueType::I32], Some(ValueType::I32)),
        ("check_nil", vec![ValueType::I32], Some(ValueType::I32)),
        ("str_equals", vec![ValueType::I32, ValueType::I32], Some(ValueType::I32)),
        ("str_not_equals", vec![ValueType::I32, ValueType::I32], Some(ValueType::I32)),
        ("str_less", vec![ValueType::I32, ValueType::I32], Some(ValueType::I32)),
        ("str_less_or_equals", vec![ValueType::I32, ValueType::I32], Some(ValueType::I32)),
        ("str_greater", vec![ValueType::I32, ValueType::I32], Some(ValueType::I32)),
        ("str_greater_or_equals", vec![ValueType::I32, ValueType::I32], Some(ValueType::I32))
	];
	let runtime : Vec<External> = vec![
	];
	let module = standard_library
		.into_iter()
		.fold(module, |mut module, (name, params, return_type)| {
			let type_index = module.push_signature(
				builder::signature()
					.with_params(params)
					.with_return_type(return_type)
				.build_sig()
			);
			module
				.import()
					.module("externals")
					.field(name)
					.external().func(type_index)
				.build()
		});
	runtime
		.into_iter()
		.fold(module, |mut module, (name, params, return_type)| {
			let type_index = module.push_signature(
				builder::signature()
					.with_params(params)
					.with_return_type(return_type)
				.build_sig()
			);
			module
				.import()
					.module("runtime")
					.field(name)
					.external().func(type_index)
				.build()
		})
}

fn emit_string(string: String, module: ModuleBuilder) -> ModuleBuilder {
	module
		.data()
			// Aca hay que ponerle el offset que vamos sumando
			.offset(I32Const(string.len().try_into().unwrap()))
			.value(string.into_bytes())
		.build()
}

fn emit_function(tree_body: Vec<Block>, frame: Frame, functions: &FunctionEnv, strings: &StringEnv, module: builder::ModuleBuilder) -> builder::ModuleBuilder {
	let (locals, params) = LocalEnv::from_frame(&frame);
	let (instructions, locals) : (Vec<Instruction>, LocalEnv) = munch_body(tree_body, locals, &functions, strings);
	module.function()
		.signature()
			.with_params(params)
			.with_return_type(Some(ValueType::I32))
			.build()
		.body()
			.with_locals(locals.finish())
			.with_instructions(Instructions::new(instructions))
			.build()
		.build()
}