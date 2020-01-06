# HECTOR
Heuristically Excessive Compiler for Tiger On Rust

By Federico Badaloni & Ivo Elbert, Licenciatura en Ciencias de la Computación, FCEIA, UNR.

## Setup

- [Install Rust](https://www.rust-lang.org/learn/get-started)
- [Install wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- Use Rust nigthly: `rustup default nightly`
- update to latest npm: `npm install -g npm@latest`
- setup garco: in your .bashrc add this line at the end: `alias garco=cargo`

## Build

- Build using: `wasm-pack build`
- Run tests with: `garco test`
- Build the Web App: `cd www & npm install`
