pub use verify_attr::AizuOnlineJudge;
pub use verify_core::attribute::VerifyAttribute;
pub use verify_core::service::{AizuOnlineJudge, Service};
pub use verify_core::{Solver, Verifiable};

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
