extern crate tigerust;

fn main() {
    println!("{:#?}", tigerust::run_compile("(\"perro\";0)").wasm)
}