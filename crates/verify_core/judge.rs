use crate::{attribute::VerifyAttribute, SolveFunc, Solver};
use std::{
    borrow::Cow,
    fmt::{Display, Formatter, Result},
    path::PathBuf,
    process::Command,
    time::Duration,
};
use tokio::time;

pub enum VerifyStatus {
    Accepted,
    WrongAnswer,
    RuntimeError,
    InternalError,
    TimeLimitExceeded,
}

#[derive(Clone, Debug)]
pub struct VerifyResult {
    pub cases: Vec<JudgeResult>,
}

impl VerifyResult {
    pub fn success(&self) -> bool {
        self.cases.iter().all(|c| c.status == JudgeStatus::Accepted)
    }

    pub fn result_icon(&self) -> &'static str {
        if self.success() {
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

pub async fn verify_inner(
    name: String,
    assertion: &StaticAssertion<'_>,
    attr: &VerifyAttribute,
    f: SolveFunc,
) -> JudgeResult {
    let mut ret = JudgeResult {
        name: name.clone(),
        status: JudgeStatus::InternalError,
        exec_time_ms: 0,
    };
    let run = async {
        let now = time::Instant::now();
        let actual = ::std::panic::catch_unwind(|| {
            let mut actual = Vec::new();
            f(&assertion.input.as_bytes(), &mut actual);
            actual
        });
        (actual, now.elapsed())
    };
    let sleep = time::sleep(Duration::from_millis(attr.time_limit_ms as u64));
    tokio::select! {
        _ = sleep => {
            // うまく動作していない 度を越えたTLEはこちらで打ち切りたい
            ret.status = JudgeStatus::TimeLimitExceeded
        },
        (actual, elapsed) = run => {
            ret.exec_time_ms = elapsed.as_millis() as u64;
            if let Ok(actual) = actual {
                match assertion.assert(&String::from_utf8_lossy(&actual)) {
                    Ok(status) => {
                        if status && ret.exec_time_ms <= attr.time_limit_ms {
                            ret.status = JudgeStatus::Accepted
                        } else if !status {
                            ret.status = JudgeStatus::WrongAnswer
                        } else {
                            ret.status = JudgeStatus::TimeLimitExceeded
                        }
                    }
                    Err(e) => {
                        println!("{:?}", e);
                        ret.status = JudgeStatus::InternalError
                    }
                }
            } else {
                ret.status = JudgeStatus::RuntimeError
            }
        },
    }
    ret
}

pub trait Assertion {
    fn assert(&self, actual: &str) -> anyhow::Result<bool>;
}

pub struct StaticAssertion<'a> {
    pub input: Cow<'a, str>,
    pub expect: Cow<'a, str>,
    pub eps: Option<f64>,
}
impl Assertion for StaticAssertion<'_> {
    fn assert(&self, actual: &str) -> anyhow::Result<bool> {
        let expect_values = self
            .expect
            .split_ascii_whitespace()
            .map(|v| v.to_string())
            .collect::<Vec<_>>();
        let actual_values = actual
            .split_ascii_whitespace()
            .map(|v| v.to_string())
            .collect::<Vec<_>>();

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

impl<'a> StaticAssertion<'a> {
    pub fn equals<S: Solver>(input: &'a str, expect: &'a str) {
        let mut buf = Vec::new();
        S::solve(input.as_bytes(), &mut buf);
        let assert = Self {
            input: Cow::Borrowed(input),
            expect: Cow::Borrowed(expect),
            eps: S::EPSILON,
        };
        assert!(assert.assert(&String::from_utf8_lossy(&buf)).expect(""))
    }
}

pub struct CheckBinaryAssertion {
    pub input_path: PathBuf,
    pub expect_path: PathBuf,
    pub checker_path: PathBuf,
}

impl Assertion for CheckBinaryAssertion {
    fn assert(&self, actual: &str) -> anyhow::Result<bool> {
        if !self.checker_path.exists() {
            println!(
                "checker file is not found {}",
                self.checker_path.to_string_lossy()
            );
        }
        let resfile = crate::save_temp_file(actual.as_bytes())?;

        let output = Command::new(self.checker_path.as_os_str())
            .args([
                self.input_path.as_os_str(),
                self.expect_path.as_os_str(),
                resfile.path().as_os_str(),
            ])
            .output()?;
        match output.status.code() {
            Some(0) => Ok(true),
            _ => Ok(false),
        }
    }
}

#[test]
fn assert_test() {
    let res = StaticAssertion {
        input: Cow::Owned("".into()),
        expect: Cow::Owned("123".into()),
        eps: None,
    }
    .assert("123");
    assert!(res.unwrap());
    let res = StaticAssertion {
        input: Cow::Owned("".into()),
        expect: Cow::Owned("123".into()),
        eps: None,
    }
    .assert("124");
    assert!(!res.unwrap());
    let res = StaticAssertion {
        input: Cow::Owned("".into()),
        expect: Cow::Owned("10000".into()),
        eps: Some(1e-4),
    }
    .assert("10001");
    assert!(res.unwrap());
    let res = StaticAssertion {
        input: Cow::Owned("".into()),
        expect: Cow::Owned("10000".into()),
        eps: Some(1e-4),
    }
    .assert("-10000");
    assert!(!res.unwrap());
    let res = StaticAssertion {
        input: Cow::Owned("".into()),
        expect: Cow::Owned("10000".into()),
        eps: Some(1e-5),
    }
    .assert("10001");
    assert!(!res.unwrap());
}
