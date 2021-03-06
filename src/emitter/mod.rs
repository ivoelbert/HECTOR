use std::convert::{TryFrom, TryInto};

mod munch;
mod enviroment;
use munch::{munch_body, };
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
		.fold(StringEnv::new(), |mut string_env, (label, string)| {string_env.insert(label.clone(), &string); string_env});
	let module = builder::module();
	let module = emit_imports(module);
	let module = strings
		.into_iter()
		// Data Section
		.fold(module, |module, (label, string)| emit_string(label, string, &string_env, module));
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
			.init_expr(I32Const(string_env.offset as i32))
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
		// .memory()
		// 	// CHEQUEAR: esto alcanza o hay que meter un grow?
		// 	.with_min(512)
		// 	.build()

		// Main Wrapper value export
		.with_export(builder::export()
			.field("tigermain_wrapper")
			.internal().func(function_env.len().try_into().unwrap())
			.build()
		)
		.import()
			.module("mem")
			.field("memory")
			.external().memory(0, None)
			.build()
		// Memory export (for the JS runtime)
		// .with_export(builder::export()
		// 	.field("memory")
		// 	.internal().memory(0)
		// 	.build()
		// )
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
    use crate::externals::*;
	type Import = (&'static str, Vec<ValueType>);
	EXTERNALS
		.iter()
		.map(|External {name, arguments, ..}| {
			(
				*name,
				arguments
					.iter()
					.map(|_| ValueType::I32)
					.collect(),
			)
		})
		.fold(module, |mut module, (name, params) : Import| {
			let type_index = module.push_signature(
				builder::signature()
					.with_params(params)
					.with_return_type(Some(ValueType::I32))
				.build_sig()
			);
			module
				.import()
					.module("externals")
					.field(name)
					.external().func(type_index)
				.build()
		})
}

fn emit_string(label: Label, mut string: String, string_env: &StringEnv, module: ModuleBuilder) -> ModuleBuilder {
	let offset = string_env.get(&label).unwrap();
	string.push('\0');
	// console_log!("Meto a la data la string <{:?}>  de bytes <{:?}> con label <{:?}> en el offset <{:?}>", &string, string.bytes(), label, (offset + string.len() as u32));
	module
		.data()
			.offset(I32Const(offset as i32))
			.value(string.into_bytes())
		.build()
}

fn emit_function(tree_body: Vec<Block>, frame: Frame, functions: &FunctionEnv, strings: &StringEnv, module: builder::ModuleBuilder) -> builder::ModuleBuilder {
	let (mut locals, params) = LocalEnv::from_frame(&frame);
	locals.insert("fp_back".to_string());
	let (instructions, locals) : (Vec<Instruction>, LocalEnv) = munch_body(tree_body, locals, &functions, strings, &frame);
	// let prologue = function_prologue(&frame);
	// let epilogue = function_epilogue();
	module.function()
		.signature()
			.with_params(params)
			.with_return_type(Some(ValueType::I32))
			.build()
		.body()
			.with_locals(locals.finish())
			.with_instructions(Instructions::new(
				vec![
					// prologue,
					instructions,
					// epilogue,
				].concat()
			))
			.build()
		.build()
}

