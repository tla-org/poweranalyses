## Build

```sh
$ rustup toolchain install stable

$ rustc --version
rustc 1.64.0 (a55dd71d5 2022-09-19)

$ rustup target add wasm32-unknown-unknown

$ cargo build --target wasm32-unknown-unknown --release
```

This build does not use `wasm-pack` meaning that it's only possible to use primitive types and also nothing from the WebAssembly System Interface (WASI).

Not using wasm-pack because SSL doesn't work.

Thanks to https://depth-first.com/articles/2020/06/29/compiling-rust-to-webassembly-a-simple-example/

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
