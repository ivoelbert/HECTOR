mod munch;
use munch::munch_stm;
extern crate parity_wasm;

use parity_wasm::builder::ModuleBuilder;
use parity_wasm::{builder, elements, elements::{Module}};
use elements::Instruction::*;
pub use parity_wasm::SerializationError;

use crate::canonization::CanonFrag;
use crate::tree::Tree;
use crate::tree::frame::*;
use crate::tree::level::Label;

pub type Wasm = Module;

pub fn emit(frags: Vec<CanonFrag>) -> (String, Vec<u8>) {
    let module : Module = frags.into_iter().fold(
        builder::module(),
        |module: ModuleBuilder, frag: CanonFrag| -> ModuleBuilder {
            match frag {
                CanonFrag::ConstString(label, string) => emit_string_global(label, string, module),
                CanonFrag::Proc{body, frame}    => emit_function(body, frame, module),
            }
        }
	)
		// STACK POINTER
		.global()
			.value_type().i32()
			.build()
		// FRAME POINTER
		.global()
			.value_type().i32()
			.build()
		// RETURN VALUE
		.global()
			.value_type().i32()
			.build()
		.build();
	println!("{:#?}", &module);
	(format!("{:?}", &module), parity_wasm::serialize(module).unwrap())
}

fn emit_string_global(label: Label, string: String, module: ModuleBuilder) -> ModuleBuilder {
	module.global()
		.value_type().i32()
		.init_expr(I32Const(42))
        // TODO
        .build()
}

fn emit_function(body: Vec<Tree::Stm>, frame: Frame, module: ModuleBuilder) -> ModuleBuilder {
	let instructions = emit_instrucions(body, &frame);
	module.function()
		.signature()
			// TODO
			.with_params(vec![])
			.with_return_type(Some(elements::ValueType::I32))
		.build()
		.body()
			// TODO
			.with_locals(vec![])
			.with_instructions(elements::Instructions::new(instructions))
		.build()
    .build()
}


fn emit_instrucions(tree_body: Vec<Tree::Stm>, frame: &Frame) -> Vec<elements::Instruction> {
	let mut instructions : Vec<elements::Instruction> = tree_body
		.into_iter()
		.map(|stm: Tree::Stm| -> Vec<elements::Instruction> {
			munch_stm(stm, frame)
		})
		.collect::<Vec<Vec<elements::Instruction>>>()
		.concat();
	instructions
}