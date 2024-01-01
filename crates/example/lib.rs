use std::io::{Read, Write};
use verify_attr::verify;

#[verify(name = "aplusb")]
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
