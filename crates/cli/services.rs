mod aizu_online_judge;
mod atcoder;
mod library_checker;
mod yukicoder;

use anyhow::anyhow;
use std::{
    ffi::OsStr,
    fs::File,
    io::{Read, Write},
    path::Path,
    process::Command,
};
use verify_core::{
    service::{
        aizu_online_judge::AizuOnlineJudge, library_checker::LibraryChecker, yukicoder::Yukicoder,
    },
    ProblemForVerify, Service,
};

pub fn save_metadata() -> anyhow::Result<()> {
    output_metadata()?;
    let root_dir = verify_core::app_cache_directory();
    for result in root_dir.read_dir()? {
        if let Ok(entry) = result {
            let path = entry.path();
            if path.extension() == Some(OsStr::new("info")) {
                if let Err(e) = normalize(&path) {
                    log::warn!("failed to normalize {}: {:?}", path.display(), e)
                }
            }
        }
    }
    Ok(())
}

fn normalize(path: &Path) -> anyhow::Result<()> {
    let mut buf = String::new();
    let mut file = File::open(&path)?;
    file.read_to_string(&mut buf)?;
    let mut v = buf.split_ascii_whitespace().collect::<Vec<_>>();
    v.sort();
    v.dedup();
    let mut file = File::create(&path)?;
    file.write_all(&v.into_iter().collect::<String>().as_bytes())?;
    Ok(())
}

fn output_metadata() -> anyhow::Result<()> {
    let output = Command::new(env!("CARGO"))
        .args([
            "test",
            "--features",
            "save_metadata",
            "--",
            "--ignored",
            "--test-threads=1",
        ])
        .output()?;
    if output.status.success() {
        Ok(())
    } else {
        Err(anyhow!("Cargo command did not finish successful."))
    }
}

pub fn fetch_testcases() -> anyhow::Result<()> {
    if let Ok(mut file) = File::open(&LibraryChecker::info_path()) {
        let mut s = String::new();
        file.read_to_string(&mut s)?;
        library_checker::fetch_problem_repository()?;
        for line in s.lines() {
            library_checker::fetch_testcases(ProblemForVerify {
                problem_id: line.to_owned(),
            })?;
        }
    }
    if let Ok(mut file) = File::open(&AizuOnlineJudge::info_path()) {
        let mut s = String::new();
        file.read_to_string(&mut s)?;
        for line in s.lines() {
            aizu_online_judge::fetch_testcases(&ProblemForVerify {
                problem_id: line.to_owned(),
            })?;
        }
    }
    if let Ok(mut file) = File::open(&Yukicoder::info_path()) {
        let mut s = String::new();
        file.read_to_string(&mut s)?;
        for line in s.lines() {
            yukicoder::fetch_testcases(&ProblemForVerify {
                problem_id: line.to_owned(),
            })?;
        }
    }
    Ok(())
}

pub fn verify() -> anyhow::Result<()> {
    Ok(())
}

fn blocking_client() -> reqwest::Result<reqwest::blocking::Client> {
    reqwest::blocking::Client::builder().build()
}
