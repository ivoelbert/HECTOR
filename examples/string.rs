extern crate hector;

fn main() {
    println!("{:#?}", hector::run_compile("(\"perro\";0)").wasm)
}