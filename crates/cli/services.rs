use anyhow::anyhow;
use std::{
    ffi::OsStr,
    fs::File,
    io::{Read, Write},
    path::Path,
    process::Command,
};

pub fn save_metadata() -> anyhow::Result<()> {
    output_metadata()?;
    let root_dir = verify_core::app_cache_directory()?;
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
    Ok(())
}

pub fn verify() -> anyhow::Result<()> {
    Ok(())
}
