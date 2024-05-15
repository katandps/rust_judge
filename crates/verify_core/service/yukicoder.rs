use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::{
    fs::{create_dir_all, File},
    io::Read,
    path::{Path, PathBuf},
};
use tokio::runtime;

use crate::{
    attribute::VerifyAttribute,
    judge::{self, JudgeResult, JudgeStatus, StaticAssertion, VerifyResult},
    Service, SolveFunc,
};

pub struct Yukicoder;

impl Service for Yukicoder {
    const SERVICE_NAME: &'static str = "yukicoder";
    fn url(problem_id: &str) -> String {
        format!("https://yukicoder.me/problems/no/{problem_id}")
    }
    fn verify(attr: VerifyAttribute, f: SolveFunc) -> anyhow::Result<VerifyResult> {
        let problem_dir =
            create_problem_directory(&attr.problem_id, &crate::app_cache_directory())?;
        let path = header_path(&problem_dir);
        let cases = YukicoderHeader::from_file(&path);
        Ok(cases.verify(&attr, &problem_dir, f))
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct YukicoderHeader {
    pub problem_id: String,
    pub list: Vec<String>,
}
impl YukicoderHeader {
    pub fn from_file(path: &PathBuf) -> Self {
        let mut buf = Vec::new();
        File::open(path)
            .expect("header file is not found")
            .read_to_end(&mut buf)
            .expect("could not load file");
        serde_json::from_slice(&buf).expect("saved header file is invalid")
    }
    fn verify(&self, attr: &VerifyAttribute, problem_dir: &Path, f: SolveFunc) -> VerifyResult {
        let cases: Vec<_> = self
            .list
            .iter()
            .map(|case_name| verify(attr, problem_dir, case_name, f))
            .collect();
        VerifyResult { cases }
    }
}

fn verify(
    attr: &VerifyAttribute,
    problem_dir: &Path,
    case_name: &str,
    f: SolveFunc,
) -> JudgeResult {
    let in_path = problem_dir.join("in").join(case_name);
    let out_path = problem_dir.join("out").join(case_name);
    let input_buf = crate::read_file(&in_path).unwrap_or_else(|_e| {
        println!("in file is not found {}:{case_name}", attr.problem_id);
        Vec::new()
    });
    let expect_buf = crate::read_file(&out_path).unwrap_or_else(|_e| {
        println!("out file is not found {}:{case_name}", attr.problem_id);
        Vec::new()
    });
    let input = String::from_utf8_lossy(&input_buf);
    let expect = String::from_utf8_lossy(&expect_buf);
    let assertion = StaticAssertion {
        input,
        expect,
        eps: attr.epsilon,
    };
    if in_path.exists() && out_path.exists() {
        runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(judge::verify_inner(
                case_name.to_string(),
                &assertion,
                attr,
                f,
            ))
    } else {
        JudgeResult {
            name: case_name.to_string(),
            status: JudgeStatus::InternalError,
            exec_time_ms: 0,
        }
    }
}

pub fn create_problem_directory(problem_id: &str, base_dir: &Path) -> anyhow::Result<PathBuf> {
    let mut problem_dir = base_dir.to_path_buf();
    problem_dir.push("yukicoder");
    problem_dir.push(problem_id);
    let in_dir = problem_dir.join("in");
    let out_dir = problem_dir.join("out");
    create_dir_all(in_dir).with_context(|| "could not create in directory")?;
    create_dir_all(out_dir).with_context(|| "could not create out directory")?;

    Ok(problem_dir)
}

pub fn header_path(problem_dir: &Path) -> PathBuf {
    problem_dir.join("header").with_extension("json")
}
