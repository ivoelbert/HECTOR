extern crate tigerust;
use std::fs::read_to_string;


fn main() {
    let contents = read_to_string("./tiger_sources/good/queens.tig").expect("read_to_string");
    println!("{:#?}", tigerust::run_compile(&contents).wasm)
}