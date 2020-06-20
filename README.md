# HECTOR

Heuristically Excessive Compiler for Tiger On Rust

By Federico Badaloni & Ivo Elbert, Licenciatura en Ciencias de la Computación, FCEIA, UNR.

## Setup

- [Install Rust](https://www.rust-lang.org/learn/get-started)
- [Install wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- [Install latest yarn](https://classic.yarnpkg.com/en/docs/install/#debian-stable)
- Install app dependencies: `cd app & yarn install`
- Build the library by running: `./buildWasm.sh`

## Test

- Run the rust-side tests with: `cargo test`
- Run the js-side interpreter and evaluator tests: `cd app & yarn test`

## Run

- Compile a tiger file with `cargo run -- path/to/file.tig`. This will output the binary WebAssembly to `output.wasm`. You can change the output with the `-o` flag. Note that this doesn't link the runtime.
- Run the app to edit and compile and run Tiger code interactively with `cd app & yarn start`.
