use reqwest::blocking;

pub fn judge() {}

pub struct TestCase {
    _input: String,
    _output: String,
}

pub struct Problem {
    _test_cases: Vec<TestCase>,
    _time_limit_ms: i64,
    _epsilon: Option<f64>,
}

pub enum VerifyStatus {
    Accepted,
    WrongAnswer,
    RuntimeError,
    InternalError,
    TimeLimitExceeded,
}
use std::fmt::{Display, Formatter, Result};

impl Display for VerifyStatus {
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

pub fn build_client() -> reqwest::Result<blocking::Client> {
    blocking::Client::builder().build()
}
