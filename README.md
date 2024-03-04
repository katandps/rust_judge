# Rust Judge Helper

## usage

add for library root file

```
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
```

and implement Solver and set service by derive macro

```
use verify::{AizuOnlineJudge, Solver};

/// # Hello World
/// <https://onlinejudge.u-aizu.ac.jp/problems/ITP1_1_A>
#[derive(AizuOnlineJudge)]
pub struct Itp1_1A;

impl Solver for Itp1_1A {
    const PROBLEM_ID: &'static str = "ITP1_1_A";
    fn solve(_read: impl Read, mut write: impl Write) {
        writeln!(write, "Hello World").ok();
    }
}
```

more examples in crates/example/lib.rs

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
