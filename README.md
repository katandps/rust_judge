# Rust Judge Helper

## commands

### fetch testcases

```sh
cargo test --features fetch_testcases -- --test-threads=1 --ignored
```

## verify

```sh
cargo test --features verify -- --ignored
```

## doc

```sh
cargo doc --workspace --no-deps --features verify_result
```
