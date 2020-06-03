// extern crate hector;
// extern crate wasmi;

// use std::fs::{read_to_string};
// use wasmi::{ModuleInstance, ImportsBuilder, NopExternals, RuntimeValue};

// #[test]
// fn wasmi_const() {
//     let wasm_binary: Vec<u8> = hector::run_compile("42").bin.unwrap();
//     // Load wasm binary and prepare it for instantiation.
//     let module = wasmi::Module::from_buffer(&wasm_binary)
//         .expect("failed to load wasm");

//     // Instantiate a module with empty imports and
//     // assert that there is no `start` function.
//     let instance =
//         ModuleInstance::new(
//             &module,
//             &ImportsBuilder::default()
//         )
//         .expect("failed to instantiate wasm module")
//         .assert_no_start();

//     // Finally, invoke the exported function "test" with no parameters
//     // and empty external function executor.
//     assert_eq!(
//         instance.invoke_export(
//             "tigermain_wrapper",
//             &[],
//             &mut NopExternals,
//         ).expect("failed to execute export"),
//         Some(RuntimeValue::I32(42)),
//     );
// }

// #[test]
// fn wasmi_addone() {
//     let contents = read_to_string("./tiger_sources/good/callAddone.tig").expect("read_to_string");
//     let wasm_binary: Vec<u8> = hector::run_compile(&contents).bin.unwrap();
//     // Load wasm binary and prepare it for instantiation.
//     let module = wasmi::Module::from_buffer(&wasm_binary)
//         .expect("failed to load wasm");

//     // Instantiate a module with empty imports and
//     // assert that there is no `start` function.
//     let instance =
//         ModuleInstance::new(
//             &module,
//             &ImportsBuilder::default()
//         )
//         .expect("failed to instantiate wasm module")
//         .assert_no_start();

//     // Finally, invoke the exported function "test" with no parameters
//     // and empty external function executor.
//     assert_eq!(
//         instance.invoke_export(
//             "tigermain_wrapper",
//             &[],
//             &mut NopExternals,
//         ).expect("failed to execute export"),
//         Some(RuntimeValue::I32(42)),
//     );
// }

// #[test]
// fn wasmi_basic_for() {
//     let contents = read_to_string("./tiger_sources/good/basicFor.tig").expect("read_to_string");
//     let wasm_binary: Vec<u8> = hector::run_compile(&contents).bin.unwrap();
//     // Load wasm binary and prepare it for instantiation.
//     let module = wasmi::Module::from_buffer(&wasm_binary)
//         .expect("failed to load wasm");

//     // Instantiate a module with empty imports and
//     // assert that there is no `start` function.
//     let instance =
//         ModuleInstance::new(
//             &module,
//             &ImportsBuilder::default()
//         )
//         .expect("failed to instantiate wasm module")
//         .assert_no_start();

//     // Finally, invoke the exported function "test" with no parameters
//     // and empty external function executor.
//     assert_eq!(
//         instance.invoke_export(
//             "tigermain_wrapper",
//             &[],
//             &mut NopExternals,
//         ).expect("failed to execute export"),
//         Some(RuntimeValue::I32(12340)),
//     );
// }

// #[test]
// fn wasmi_basic_while() {
//     let contents = read_to_string("./tiger_sources/good/basicWhile.tig").expect("read_to_string");
//     let wasm_binary: Vec<u8> = hector::run_compile(&contents).bin.unwrap();
//     // Load wasm binary and prepare it for instantiation.
//     let module = wasmi::Module::from_buffer(&wasm_binary)
//         .expect("failed to load wasm");

//     // Instantiate a module with empty imports and
//     // assert that there is no `start` function.
//     let instance =
//         ModuleInstance::new(
//             &module,
//             &ImportsBuilder::default()
//         )
//         .expect("failed to instantiate wasm module")
//         .assert_no_start();

//     // Finally, invoke the exported function "test" with no parameters
//     // and empty external function executor.
//     assert_eq!(
//         instance.invoke_export(
//             "tigermain_wrapper",
//             &[],
//             &mut NopExternals,
//         ).expect("failed to execute export"),
//         Some(RuntimeValue::I32(10)),
//     );
// }

// #[test]
// fn wasmi_identity() {
//     let contents = read_to_string("./tiger_sources/good/callIdentity.tig").expect("read_to_string");
//     let wasm_binary: Vec<u8> = hector::run_compile(&contents).bin.unwrap();
//     // Load wasm binary and prepare it for instantiation.
//     let module = wasmi::Module::from_buffer(&wasm_binary)
//         .expect("failed to load wasm");

//     // Instantiate a module with empty imports and
//     // assert that there is no `start` function.
//     let instance =
//         ModuleInstance::new(
//             &module,
//             &ImportsBuilder::default()
//         )
//         .expect("failed to instantiate wasm module")
//         .assert_no_start();

//     // Finally, invoke the exported function "test" with no parameters
//     // and empty external function executor.
//     assert_eq!(
//         instance.invoke_export(
//             "tigermain_wrapper",
//             &[],
//             &mut NopExternals,
//         ).expect("failed to execute export"),
//         Some(RuntimeValue::I32(42)),
//     );
// }

// #[test]
// fn wasmi_fact() {
//     let contents = read_to_string("./tiger_sources/good/fact.tig").expect("read_to_string");
//     let wasm_binary: Vec<u8> = hector::run_compile(&contents).bin.unwrap();
//     // Load wasm binary and prepare it for instantiation.
//     let module = wasmi::Module::from_buffer(&wasm_binary)
//         .expect("failed to load wasm");

//     // Instantiate a module with empty imports and
//     // assert that there is no `start` function.
//     let instance =
//         ModuleInstance::new(
//             &module,
//             &ImportsBuilder::default()
//         )
//         .expect("failed to instantiate wasm module")
//         .assert_no_start();

//     // Finally, invoke the exported function "test" with no parameters
//     // and empty external function executor.
//     assert_eq!(
//         instance.invoke_export(
//             "tigermain_wrapper",
//             &[],
//             &mut NopExternals,
//         ).expect("failed to execute export"),
//         Some(RuntimeValue::I32(120)),
//     );
// }

// #[test]
// fn wasmi_local_hide_global() {
//     let contents = read_to_string("./tiger_sources/good/localHideGlobal.tig").expect("read_to_string");
//     let wasm_binary: Vec<u8> = hector::run_compile(&contents).bin.unwrap();
//     // Load wasm binary and prepare it for instantiation.
//     let module = wasmi::Module::from_buffer(&wasm_binary)
//         .expect("failed to load wasm");

//     // Instantiate a module with empty imports and
//     // assert that there is no `start` function.
//     let instance =
//         ModuleInstance::new(
//             &module,
//             &ImportsBuilder::default()
//         )
//         .expect("failed to instantiate wasm module")
//         .assert_no_start();

//     // Finally, invoke the exported function "test" with no parameters
//     // and empty external function executor.
//     assert_eq!(
//         instance.invoke_export(
//             "tigermain_wrapper",
//             &[],
//             &mut NopExternals,
//         ).expect("failed to execute export"),
//         Some(RuntimeValue::I32(2)),
//     );
// }

// #[test]
// fn wasmi_return_variable() {
//     let contents = read_to_string("./tiger_sources/good/returnVariable.tig").expect("read_to_string");
//     let wasm_binary: Vec<u8> = hector::run_compile(&contents).bin.unwrap();
//     // Load wasm binary and prepare it for instantiation.
//     let module = wasmi::Module::from_buffer(&wasm_binary)
//         .expect("failed to load wasm");

//     // Instantiate a module with empty imports and
//     // assert that there is no `start` function.
//     let instance =
//         ModuleInstance::new(
//             &module,
//             &ImportsBuilder::default()
//         )
//         .expect("failed to instantiate wasm module")
//         .assert_no_start();

//     // Finally, invoke the exported function "test" with no parameters
//     // and empty external function executor.
//     assert_eq!(
//         instance.invoke_export(
//             "tigermain_wrapper",
//             &[],
//             &mut NopExternals,
//         ).expect("failed to execute export"),
//         Some(RuntimeValue::I32(42)),
//     );
// }

// #[test]
// fn wasmi_if() {
//     let contents = read_to_string("./tiger_sources/good/test08.tig").expect("read_to_string");
//     let wasm_binary: Vec<u8> = hector::run_compile(&contents).bin.unwrap();
//     // Load wasm binary and prepare it for instantiation.
//     let module = wasmi::Module::from_buffer(&wasm_binary)
//         .expect("failed to load wasm");

//     // Instantiate a module with empty imports and
//     // assert that there is no `start` function.
//     let instance =
//         ModuleInstance::new(
//             &module,
//             &ImportsBuilder::default()
//         )
//         .expect("failed to instantiate wasm module")
//         .assert_no_start();

//     // Finally, invoke the exported function "test" with no parameters
//     // and empty external function executor.
//     assert_eq!(
//         instance.invoke_export(
//             "tigermain_wrapper",
//             &[],
//             &mut NopExternals,
//         ).expect("failed to execute export"),
//         Some(RuntimeValue::I32(40)),
//     );
// }