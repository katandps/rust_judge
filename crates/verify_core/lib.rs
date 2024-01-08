pub mod attribute;
pub mod judge;
pub mod service;

use anyhow::Error;
use serde::Deserialize;
use std::process::Command;

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
