use std::io::{Read, Write};
#[verify::aizu_online_judge(problem_id = "hoge", eps = 1e-6)]
fn solve(mut read: impl Read, mut write: impl Write) {
    let mut buf = String::new();
    read.read_to_string(&mut buf).ok();
    write!(write, "{buf}").ok();
}

#[test]
fn test() {
    let mut result = Vec::new();
    solve("test".as_bytes(), &mut result);
    assert_eq!("test".as_bytes(), result);
}
