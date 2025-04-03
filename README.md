# QuestDB configuration string parser

This is for:
* The [questdb-confstr](./questdb-confstr) crate
* and its bindings for C [questdb-confstr-ffi](./questdb-confstr-ffi)

## Dev notes

## Build and test

```
cargo clean
cargo build
cargo test
cd questdb-confstr-ffi/cpp_test
./compile
./run
```

## Linting

In addition, before pushing a PR, please run:
```
cargo fmt --all
cargo clippy --all-targets -- -D warnings
```

## Cutting a release

Let us assume we want to cut version `0.1.1` and the current version is `0.1.0`.

1. New branch
```shell
git switch -c v0.1.1
```

2. Update the version in both `Cargo.toml` files

```shell
$EDITOR questdb-confstr/Cargo.toml
$EDITOR questdb-confstr-ffi/Cargo.toml
```

3. Re-run all tests and lints

Don't forget about _clippy_: The newest version of Rust might have picked up
some new lints that we need to fix.

4. Commit the changes and create a PR
```shell
git add -u
git commit -m "Bump version to 0.1.1"
git push --set-upstream origin v0.1.1
```

N.B: _We don't need to update any `Cargo.lock` files for the ffi crate since
there are no dependencies._

5. Create a new PR, get it reviewed and merge it

https://github.com/questdb/questdb-confstr-rs/pull/new/

6. Publish both crates to crates.io

```shell
cargo login

(cd questdb-confstr && cargo publish --dry-run)
(cd questdb-confstr-ffi && cargo publish --dry-run)

(cd questdb-confstr && cargo publish)
(cd questdb-confstr && cargo publish)
```

(If in doubt, see the
"[Publishing on crates.io](https://doc.rust-lang.org/cargo/reference/publishing.html)" guide)