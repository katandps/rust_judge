use std::io::{Read, Write};
#[verify::aizu_online_judge(problem_id = "ITP1_1_A")]
fn solve(_read: impl Read, mut write: impl Write) {
    writeln!(write, "Hello World").ok();
    write.flush().ok();
}

#[test]
fn test() {
    let mut result = Vec::new();
    solve("test".as_bytes(), &mut result);
    assert_eq!("test".as_bytes(), result);
}
