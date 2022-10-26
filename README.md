# poweranalyses.org

## Developer notes

Going via emscripten because we link a C library.

A big thanks to https://github.com/rustwasm/team/issues/291#issuecomment-644946504 for writing down how to build a C library to WebAssembly via Rust.

### Test

```sh
$ cargo test
```

Or, to test whenever files change:

```sh
$ find src | entr -s 'cargo test'
```

### Serve

To rebuild when files change:

```sh
$ ./entr.sh
```

To serve the website:

```sh
$ ./serve.sh
```
