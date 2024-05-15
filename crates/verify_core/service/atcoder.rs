use serde::{Deserialize, Serialize};
use std::{
    fs::{create_dir_all, File},
    io::Read,
    path::{Path, PathBuf},
};

use anyhow::Context;

use crate::{judge::VerifyResult, Service};

pub struct AtCoder;
impl Service for AtCoder {
    const SERVICE_NAME: &'static str = "atcoder";
    fn url(_problem_id: &str) -> String {
        "https://atcoder.jp/".to_string()
    }
    fn verify(
        attr: crate::attribute::VerifyAttribute,
        _f: crate::SolveFunc,
    ) -> anyhow::Result<crate::judge::VerifyResult> {
        let problem_dir =
            create_problem_directory(&attr.problem_id, &crate::app_cache_directory())?;
        let path = header_path(&problem_dir);
        let _cases = AtCoderHeader::from_file(&path);
        // Ok(cases.verify(&attr, &problem_dir, f))
        Ok(VerifyResult { cases: Vec::new() })
        // }
    }
}

fn create_problem_directory(problem_id: &str, base_dir: &Path) -> anyhow::Result<PathBuf> {
    let mut problem_dir = base_dir.to_path_buf();
    problem_dir.push("atcoder");
    problem_dir.push(problem_id);
    let in_dir = problem_dir.join("in");
    let out_dir = problem_dir.join("out");
    create_dir_all(in_dir).with_context(|| "could not create in directory")?;
    create_dir_all(out_dir).with_context(|| "could not create out directory")?;

    Ok(problem_dir)
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AtCoderHeader {
    pub problem_id: String,
    pub list: Vec<String>,
}
pub fn header_path(problem_dir: &Path) -> PathBuf {
    problem_dir.join("header").with_extension("json")
}

impl AtCoderHeader {
    fn from_file(path: &PathBuf) -> Self {
        let mut buf = Vec::new();
        File::open(path)
            .expect("header file is not found")
            .read_to_end(&mut buf)
            .expect("could not load file");
        serde_json::from_slice(&buf).expect("saved header file is invalid")
    }
}
