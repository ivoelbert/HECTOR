extern crate tigerust;
use std::fs::read_to_string;


fn main() {
    let contents = read_to_string("./tiger_sources/good/basicFor.tig").expect("read_to_string");
    tigerust::run_compile(&contents);
}