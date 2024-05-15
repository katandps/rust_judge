pub mod attribute;
pub mod judge;
pub mod service;

use anyhow::Error;
use attribute::VerifyAttribute;
use chrono::SecondsFormat;
use dirs::cache_dir;
use judge::VerifyResult;
use serde::Deserialize;
use std::{
    borrow::Cow,
    env::temp_dir,
    fs::{create_dir_all, File, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
    process::Command,
    str::FromStr,
};
use tempfile::NamedTempFile;

use crate::judge::{Assertion, StaticAssertion};

const APP_NAME: &str = "rust_judge";

type SolveFunc = fn(&[u8], &mut Vec<u8>);

pub trait Service {
    fn fetch_testcases(problem_id: &str) -> anyhow::Result<()>;
    fn verify(attr: VerifyAttribute, f: SolveFunc) -> anyhow::Result<VerifyResult>;
    fn url(problem_id: &str) -> String;
    const SERVICE_NAME: &'static str;
}

pub trait Solver {
    const PROBLEM_ID: &'static str;
    const EPSILON: Option<f64> = None;
    const TIME_LIMIT_MILLIS: u64 = 10000;
    fn solve(read: impl Read, write: impl Write);
    fn assert(input: &str, expect: &str) {
        let mut buf = Vec::new();
        Self::solve(input.as_bytes(), &mut buf);
        let assert = StaticAssertion {
            input: Cow::Borrowed(input),
            expect: Cow::Borrowed(expect),
            eps: Self::EPSILON,
        };
        assert!(assert.assert(&String::from_utf8_lossy(&buf)).expect(""))
    }
}

pub trait Verifiable: Solver {
    type SERVICE: Service;

    fn save_metadata() -> anyhow::Result<()> {
        let mut root = app_cache_directory()?;
        root.push(Self::SERVICE::SERVICE_NAME);
        root.set_extension("info");
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(root)?
            .write(format!("{}\n", Self::PROBLEM_ID).as_bytes())?;
        Ok(())
    }

    fn fetch_testcases() {
        if let Err(e) = Self::SERVICE::fetch_testcases(Self::PROBLEM_ID) {
            panic!("{:?}", e)
        }
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
        File::create(md_path)?.write_all(Self::generate_md(res).as_bytes())?;
        Ok(())
    }

    fn generate_md(res: &VerifyResult) -> String {
        let (icon, url, tl) = (
            res.result_icon(),
            Self::SERVICE::url(Self::PROBLEM_ID),
            Self::TIME_LIMIT_MILLIS,
        );
        let mut body = String::new();
        for case in &res.cases {
            body.push_str(&format!(
                "| {} | {} | {}ms |\n",
                case.name, case.status, case.exec_time_ms
            ));
        }
        let footer = format!(
            "this document generated in {}",
            chrono::Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true)
        );
        format!(
            "# Verify Result {icon}\n
## [PROBLEM LINK]({url})\n
TL: {tl}ms\n
| case name | judge | elapsed time |
| :--- | :---: | ---: |
{body}\n{footer}\n",
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

pub fn app_cache_directory() -> anyhow::Result<PathBuf> {
    let mut path = cache_dir().unwrap_or_else(temp_dir);
    path.push(crate::APP_NAME);
    create_dir_all(&path)?;
    Ok(path)
}

fn blocking_client() -> reqwest::Result<reqwest::blocking::Client> {
    reqwest::blocking::Client::builder().build()
}

fn read_file(path: &PathBuf) -> anyhow::Result<Vec<u8>> {
    let mut buf = Vec::new();
    File::open(path)?.read_to_end(&mut buf)?;
    Ok(buf)
}

fn save_temp_file(buf: &[u8]) -> anyhow::Result<NamedTempFile> {
    let mut file = NamedTempFile::new()?;
    file.write_all(buf)?;
    Ok(file)
}
