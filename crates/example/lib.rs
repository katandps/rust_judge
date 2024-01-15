use std::io::{Read, Write};
use verify::{AizuOnlineJudge, Solver};

#[test]
fn test() {
    let mut result = Vec::new();
    Itp1_1A::solve("".as_bytes(), &mut result);
    assert_eq!("Hello World\n".as_bytes(), result);
}

/// # Hello World
/// <https://onlinejudge.u-aizu.ac.jp/problems/ITP1_1_A>
#[derive(AizuOnlineJudge)]
pub struct Itp1_1A;

impl Solver for Itp1_1A {
    const PROBLEM_ID: &'static str = "ITP1_1_A";
    fn solve(_read: impl Read, mut write: impl Write) {
        writeln!(write, "Hello World").ok();
        write.flush().ok();
    }
}

#[derive(AizuOnlineJudge)]
pub struct Itp1_1aTLE;
impl Solver for Itp1_1aTLE {
    const PROBLEM_ID: &'static str = "ITP1_1_A";
    const TIME_LIMIT_MILLIS: u64 = 100;
    fn solve(_read: impl Read, mut write: impl Write) {
        std::thread::sleep(std::time::Duration::from_secs(1));
        writeln!(write, "Hello World").ok();
        write.flush().ok();
    }
}
