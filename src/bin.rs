extern crate clap;
use clap::{Arg, App};

use std::fs::{File, read_to_string};
use std::io::prelude::*;
use std::path::Path;

use hector::run_compile;

pub fn main() -> Result<(), String> {
    let matches = App::new("HECTOR")
                          .version("1.0")
                          .author("Federico Badaloni, Iván Elbert.")
                          .about("Heuristically Excesive Compiler for Tiger Over Rust")
                          .arg(Arg::with_name("INPUT")
                               .help("The tiger file to compile")
                               .required(true)
                               .index(1))
                          .arg(Arg::with_name("OUTPUT")
                               .short("o")
                               .long("output")
                               .takes_value(true)
                               .help("The output file to save the compiled code. Defaults to 'output.wasm'.")
                               .required(false))
                          .get_matches();

    let input = matches.value_of("INPUT").unwrap();
    if !input.ends_with(".tig") {
        return Err("Please use a .tig file as input.".to_string());
    }
    let output = if let Some(file) = matches.value_of("OUTPUT") {
        file
    } else {
        "output.wasm"
    };

    let input_path = Path::new(input);
    let output_path = Path::new(output);

    let source_code = match read_to_string(input_path) {
        Ok(source_code) => source_code,
        Err(why) => return Err(format!("couldn't read {}: {}", input_path.display(), why)),
    };

    let mut output_file = match File::create(&output_path) {
        Err(why) => return Err(format!("couldn't create {}: {}", output_path.display(), why)),
        Ok(file) => file,
    };

    let compiled = run_compile(&source_code);

    let bytes = if let Err(parse_error) = compiled.parse {
        return Err(format!("Parse Error: {:?}", parse_error))
    } else if let Some(Err(typecheck_error)) = compiled.typecheck {
        return Err(format!("Type Error: {:?}", typecheck_error))
    } else if let Some(Err(translate_error)) = compiled.translate {
        return Err(format!("Translation Error: {:?}", translate_error))
    } else {
        compiled.bin.unwrap()
    };

    match output_file.write_all(&bytes) {
        Err(why) => Err(format!("couldn't write to {}: {}", output_path.display(), why)),
        Ok(_) => Ok(()),
    }

}