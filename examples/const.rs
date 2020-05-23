extern crate hector;

fn main() {
    println!("{:#?}", hector::run_compile("42").wasm)
}