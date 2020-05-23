extern crate hector;

fn main() {
    println!("{:#?}", hector::run_compile("2+2").wasm)
}