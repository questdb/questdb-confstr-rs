# QuestDB configuration string parser

This is for:
* The [questdb-confstr](./questdb-confstr) crate
* and its bindings for C [questdb-confstr-ffi](./questdb-confstr-ffi)

## Dev notes

To build and test:

```
cargo clean
cargo build
cargo test
cd questdb-confstr-ffi/cpp_test
./compile
./run
```

In addition, before pushing a PR, please run:
```
cargo fmt --all
cargo clippy --all-targets -- -D warnings
```