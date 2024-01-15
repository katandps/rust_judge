pub mod attribute;
pub mod judge;
pub mod service;

use anyhow::Error;
use attribute::VerifyAttribute;
use dirs::cache_dir;
use judge::VerifyResult;
use serde::Deserialize;
use service::Service;
use std::{
    env::temp_dir,
    fs::File,
    io::{Read, Write},
    path::PathBuf,
    process::Command,
};

const APP_NAME: &'static str = "rust_judge";

pub trait Solver {
    const PROBLEM_ID: &'static str;
    const EPSILON: f64 = 0f64;
    const TIME_LIMIT: f64 = 10.0f64;
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
                time_limit: Self::TIME_LIMIT,
            },
            Self::verify_inner,
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

fn target_directory() -> anyhow::Result<String> {
    #[derive(Debug, Clone, Deserialize)]
    struct TargetDir {
        target_directory: String,
    }
    let output = Command::new(env!("CARGO"))
        .args(["metadata", "--quiet", "--no-deps"])
        .output()?;

    if output.status.success() {
        Ok(serde_json::from_slice::<TargetDir>(&output.stdout)?.target_directory)
    } else {
        Err(Error::msg("Cargo command did not finish successful."))
    }
}
pub fn load_verify_info<S: Service>() -> anyhow::Result<Vec<VerifyAttribute>> {
    let mut target = PathBuf::from(target_directory()?);
    target.push(APP_NAME);
    target.push(S::SERVICE_NAME);
    log::info!("start loading {}", target.to_string_lossy());
    if target.exists() && target.is_dir() {
        let mut result = Vec::new();
        for entry in target.read_dir()? {
            if let Ok(entry) = entry {
                let mut buf = Vec::new();
                File::open(&target.join(entry.file_name()))?.read_to_end(&mut buf)?;
                result.push(serde_json::from_slice(&buf)?);
            }
        }
        Ok(result)
    } else {
        Ok(Vec::new())
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
