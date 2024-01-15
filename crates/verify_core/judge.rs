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
