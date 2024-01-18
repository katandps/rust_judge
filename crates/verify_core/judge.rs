pub enum VerifyStatus {
    Accepted,
    WrongAnswer,
    RuntimeError,
    InternalError,
    TimeLimitExceeded,
}
use std::{
    fmt::{Display, Formatter, Result},
    io::Read,
};

#[derive(Clone, Debug)]
pub struct VerifyResult {
    pub success: bool,
    pub cases: Vec<JudgeResult>,
}

impl VerifyResult {
    pub fn result_icon(&self) -> &'static str {
        if self.success {
            "✅"
        } else {
            "❌"
        }
    }
}

#[derive(Clone, Debug)]
pub struct JudgeResult {
    pub status: JudgeStatus,
    pub name: String,
    pub exec_time_ms: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JudgeStatus {
    Accepted,
    WrongAnswer,
    RuntimeError,
    TimeLimitExceeded,
    InternalError,
}
impl Display for JudgeStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Accepted => write!(f, "AC"),
            Self::WrongAnswer => write!(f, "WA"),
            Self::RuntimeError => write!(f, "RE"),
            Self::InternalError => write!(f, "IE"),
            Self::TimeLimitExceeded => write!(f, "TLE"),
        }
    }
}

pub trait Assertion {
    fn assert(&self, expect: impl Read, actual: impl Read) -> anyhow::Result<bool>;
}

pub struct StaticAssertion {
    pub eps: Option<f64>,
}
impl Assertion for StaticAssertion {
    fn assert(&self, mut expect: impl Read, mut actual: impl Read) -> anyhow::Result<bool> {
        let (mut actual_values, mut expect_values) = (Vec::new(), Vec::new());
        {
            let mut buf = String::new();
            expect.read_to_string(&mut buf)?;
            for v in buf.split_ascii_whitespace() {
                expect_values.push(v.to_string());
            }
        }
        {
            let mut buf = String::new();
            actual.read_to_string(&mut buf)?;
            for v in buf.split_ascii_whitespace() {
                actual_values.push(v.to_string());
            }
        }
        if expect_values.len() != actual_values.len() {
            println!("expect: {:?}\nactual: {:?}", expect_values, actual_values);
            Ok(false)
        } else {
            for (expect, actual) in expect_values.iter().zip(&actual_values) {
                if let Some(eps) = self.eps {
                    match (expect.parse::<f64>(), actual.parse::<f64>()) {
                        (Ok(ex), Ok(ac)) => {
                            if !((ex - ac).abs() <= eps || ((ex - ac) / ex).abs() <= eps) {
                                println!(
                                    "expect: {:?}\nactual: {:?}",
                                    expect_values, actual_values
                                );
                                return Ok(false);
                            }
                        }
                        _ => {
                            if expect != actual {
                                println!(
                                    "expect: {:?}\nactual: {:?}",
                                    expect_values, actual_values
                                );
                                return Ok(false);
                            }
                        }
                    }
                } else {
                    if expect != actual {
                        println!("expect: {:?}\nactual: {:?}", expect_values, actual_values);
                        return Ok(false);
                    }
                }
            }
            Ok(true)
        }
    }
}

#[test]
fn assert_test() {
    let res = StaticAssertion { eps: None }.assert("123".as_bytes(), "123".as_bytes());
    assert!(res.unwrap());
    let res = StaticAssertion { eps: None }.assert("123".as_bytes(), "124".as_bytes());
    assert!(!res.unwrap());
    let res = StaticAssertion { eps: Some(1e-4) }.assert("10000".as_bytes(), "10001".as_bytes());
    assert!(res.unwrap());
    let res = StaticAssertion { eps: Some(1e-4) }.assert("10000".as_bytes(), "-10000".as_bytes());
    assert!(!res.unwrap());
    let res = StaticAssertion { eps: Some(1e-5) }.assert("10000".as_bytes(), "10001".as_bytes());
    assert!(!res.unwrap());
}
