extern crate tigerust;

fn main() {
    println!("{:#?}", tigerust::run_compile("42").wasm)
}