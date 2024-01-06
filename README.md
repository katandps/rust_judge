# Rust Judge Helper

## includes

### verify

command for verify

### verify_attr

下記の Attribute を提供する

```rust
#[verify::library_checker(problem_id = "aplusb", eps = 1e6, tl = 2.0)]
fn solve(read: impl Read, write: impl Write) {
}
```

### verify_core

上記ライブラリのコア実装
