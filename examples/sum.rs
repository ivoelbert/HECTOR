extern crate tigerust;

fn main() {
    println!("{:#?}", tigerust::run_compile("2+2").wasm)
}