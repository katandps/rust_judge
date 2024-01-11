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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JudgeResult {
    Accepted,
    WrongAnswer,
    RuntimeError,
    TimeLimitExceeded,
    InternalError,
}
impl Display for JudgeResult {
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
