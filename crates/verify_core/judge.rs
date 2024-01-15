pub enum VerifyStatus {
    Accepted,
    WrongAnswer,
    RuntimeError,
    InternalError,
    TimeLimitExceeded,
}
use std::{
    fmt::{Display, Formatter, Result},
    fs::File,
    io::Write,
    path::PathBuf,
    str::FromStr,
};

#[derive(Clone, Debug)]
pub struct VerifyResult {
    pub success: bool,
    pub cases: Vec<JudgeResult>,
}

impl VerifyResult {
    pub fn output(&self, path: &str, ident: &str) -> anyhow::Result<()> {
        let mut md_path = PathBuf::from_str(&crate::workspace_root_directory()?)?;
        md_path.push(path);
        md_path.pop();
        md_path.push(format!("result_{ident}.md"));
        log::info!("{:?}", md_path);
        File::create(md_path)?.write_all(self.generate_md().as_bytes())?;
        Ok(())
    }

    fn generate_md(&self) -> String {
        let mut body = String::new();
        for case in &self.cases {
            body.push_str(&format!(
                "| {} | {} | {}ms |\n",
                case.name, case.status, case.exec_time_ms
            ));
        }
        format!(
            "# Verify Result {}\n\n| case name | judge | elapsed time |\n| --- | --- | --- |\n{}",
            self.icon(),
            body
        )
    }

    fn icon(&self) -> &'static str {
        if self.success {
            "[x]"
        } else {
            "[ ]"
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
