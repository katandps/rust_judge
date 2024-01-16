pub enum VerifyStatus {
    Accepted,
    WrongAnswer,
    RuntimeError,
    InternalError,
    TimeLimitExceeded,
}
use std::fmt::{Display, Formatter, Result};

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

pub struct StaticAssertion;
impl StaticAssertion {
    pub fn assert(
        mut expect: impl std::io::Read,
        mut actual: impl std::io::Read,
        _eps: f64,
    ) -> anyhow::Result<bool> {
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
        if expect_values == actual_values {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
