pub mod attribute;
pub mod judge;
pub mod service;

use anyhow::Error;
use attribute::VerifyAttribute;
use dirs::cache_dir;
use judge::VerifyResult;
use serde::Deserialize;
use std::{
    env::temp_dir,
    fs::File,
    io::{Read, Write},
    path::PathBuf,
    process::Command,
    str::FromStr,
};

const APP_NAME: &'static str = "rust_judge";

type SolveFunc = fn(&[u8], &mut Vec<u8>);

pub trait Service {
    fn fetch_testcases(problem_id: &str) -> anyhow::Result<()>;
    fn verify(attr: VerifyAttribute, f: SolveFunc) -> anyhow::Result<VerifyResult>;
    fn url(problem_id: &str) -> String;
    const SERVICE_NAME: &'static str;
}

pub trait Solver {
    const PROBLEM_ID: &'static str;
    const EPSILON: f64 = 0f64;
    const TIME_LIMIT_MILLIS: u64 = 10000;
    fn solve(read: impl Read, write: impl Write);
}

pub trait Verifiable: Solver {
    type SERVICE: Service;

    fn fetch_testcases() {
        Self::SERVICE::fetch_testcases(Self::PROBLEM_ID).expect("failed to fetch testcases");
    }
    fn verify_inner(read: &[u8], write: &mut Vec<u8>) {
        Self::solve(read, write)
    }
    fn verify() -> anyhow::Result<VerifyResult> {
        Self::SERVICE::verify(
            VerifyAttribute {
                problem_id: Self::PROBLEM_ID.to_string(),
                epsilon: Self::EPSILON,
                time_limit_ms: Self::TIME_LIMIT_MILLIS,
            },
            Self::verify_inner,
        )
    }
    fn output(res: &VerifyResult, path: &str, ident: &str) -> anyhow::Result<()> {
        let mut md_path = PathBuf::from_str(&crate::workspace_root_directory()?)?;
        md_path.push(path);
        md_path.pop();
        md_path.push(format!("result_{ident}.md"));
        println!("{:?}", md_path);
        File::create(md_path)?.write_all(Self::generate_md(res).as_bytes())?;
        Ok(())
    }

    fn generate_md(res: &VerifyResult) -> String {
        let mut body = String::new();
        for case in &res.cases {
            body.push_str(&format!(
                "| {} | {} | {}ms |\n",
                case.name, case.status, case.exec_time_ms
            ));
        }
        format!(
            "# Verify Result {}\n\n## [PROBLEM LINK]({})\n\n\nTL: {}ms\n\n| case name | judge | elapsed time |\n| --- | --- | --- |\n{}",
            res.result_icon(),
            Self::SERVICE::url(Self::PROBLEM_ID),
            Self::TIME_LIMIT_MILLIS,
            body
        )
    }
}

fn workspace_root_directory() -> anyhow::Result<String> {
    #[derive(Debug, Clone, Deserialize)]
    struct TargetDir {
        workspace_root: String,
    }
    let output = Command::new(env!("CARGO"))
        .args(["metadata", "--quiet", "--no-deps"])
        .output()?;

    if output.status.success() {
        Ok(serde_json::from_slice::<TargetDir>(&output.stdout)?.workspace_root)
    } else {
        Err(Error::msg("Cargo command did not finish successful."))
    }
}

fn app_cache_directory() -> PathBuf {
    let mut path = cache_dir().unwrap_or_else(|| temp_dir());
    path.push(crate::APP_NAME);
    path
}

fn blocking_client() -> reqwest::Result<reqwest::blocking::Client> {
    reqwest::blocking::Client::builder().build()
}
