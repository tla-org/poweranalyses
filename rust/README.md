## Build

Going via emscripten because we link a C library.

```sh
$ rustup toolchain install stable

$ rustc --version
rustc 1.64.0 (a55dd71d5 2022-09-19)

$ rustup target add wasm32-unknown-emscripten # wasm32-wasi

$ cargo build --target wasm32-unknown-emscripten --release
```

This build does not use `wasm-pack` meaning that it's only possible to use primitive types and also nothing/not much from the WebAssembly System Interface (WASI).

Thanks to https://depth-first.com/articles/2020/06/29/compiling-rust-to-webassembly-a-simple-example/
Also big thanks to https://github.com/rustwasm/team/issues/291#issuecomment-644946504.

## Test

```sh
$ cargo test
```

Or, to test whenever files change:

```sh
$ find src | entr -s 'cargo test'
```

## Serve

```sh
$ julia -ie 'using LiveServer; serve()'
```
