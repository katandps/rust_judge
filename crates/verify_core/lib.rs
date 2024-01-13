pub mod attribute;
pub mod judge;
pub mod service;

use anyhow::Error;
use attribute::VerifyAttribute;
use dirs::cache_dir;
use serde::Deserialize;
use service::Service;
use std::{env::temp_dir, fs::File, io::Read, path::PathBuf, process::Command};

const APP_NAME: &'static str = "rust_judge";

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
