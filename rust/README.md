## Build

```sh
$ rustup toolchain install stable

$ rustup target add wasm32-unknown-unknown

$ cargo build --target wasm32-unknown-unknown --release
```

This build does not use `wasm-pack` meaning that it's only possible to use primitive types and also nothing from the WebAssembly System Interface (WASI).

Not using wasm-pack because SSL doesn't work.

## Serve

```sh
$ julia -ie 'using LiveServer; serve()'
```
