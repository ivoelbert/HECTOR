extern crate tigerust;
use std::fs::read_to_string;


fn main() {
    println!("{:#?}", tigerust::run_compile("42").wasm)
}