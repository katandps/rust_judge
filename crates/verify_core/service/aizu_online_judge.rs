use crate::{
    attribute::VerifyAttribute,
    judge::{self, JudgeResult, JudgeStatus, StaticAssertion, VerifyResult},
    Service, SolveFunc,
};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read, path::PathBuf};
use tokio::runtime::Builder;

#[derive(Deserialize, Serialize, Debug)]
pub struct AOJTestCaseHeaders {
    // problemId: String,
    pub headers: Vec<AOJTestCaseHeader>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AOJTestCaseHeader {
    pub serial: u32,
    name: String,
    // inputSize: i64,
    // outputSize:i64,
    // score: i64,
}

pub struct AizuOnlineJudge;

impl Service for AizuOnlineJudge {
    fn url(problem_id: &str) -> String {
        format!("https://onlinejudge.u-aizu.ac.jp/problems/{problem_id}")
    }

    fn verify(attr: VerifyAttribute, f: SolveFunc) -> anyhow::Result<VerifyResult> {
        let mut buf = Vec::new();
        File::open(Self::header_path(&attr.problem_id)?)?.read_to_end(&mut buf)?;
        let headers: AOJTestCaseHeaders = serde_json::from_slice(&buf)?;
        Ok(headers.verify(&attr, f))
    }
    const SERVICE_NAME: &'static str = "aizu_online_judge";
}

impl AizuOnlineJudge {
    fn problem_dir_path(problem_id: &str) -> anyhow::Result<PathBuf> {
        let mut problem_dir = crate::app_cache_directory();
        problem_dir.push(Self::SERVICE_NAME);
        problem_dir.push(problem_id);
        Ok(problem_dir)
    }
    pub fn header_path(problem_id: &str) -> anyhow::Result<PathBuf> {
        Ok(Self::problem_dir_path(problem_id)?
            .join("header")
            .with_extension("json"))
    }
}

impl AOJTestCaseHeaders {
    fn verify(&self, attr: &VerifyAttribute, f: SolveFunc) -> VerifyResult {
        let cases: Vec<_> = self
            .headers
            .iter()
            .map(|header| {
                header.verify(attr, f).unwrap_or(JudgeResult {
                    name: header.name.clone(),
                    status: JudgeStatus::InternalError,
                    exec_time_ms: 0,
                })
            })
            .collect();
        VerifyResult { cases }
    }
}

impl AOJTestCaseHeader {
    fn verify(&self, attr: &VerifyAttribute, f: SolveFunc) -> anyhow::Result<JudgeResult> {
        let in_path = self.in_path(&attr.problem_id)?;
        let out_path = self.out_path(&attr.problem_id)?;
        let input_buf = crate::read_file(&in_path).unwrap_or_else(|_e| {
            println!("in file is not found {}:{}", attr.problem_id, self.name);
            Vec::new()
        });
        let expect_buf = crate::read_file(&out_path).unwrap_or_else(|_e| {
            println!("out file is not found {}:{}", attr.problem_id, self.name);
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
            Ok(Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(judge::verify_inner(self.name.clone(), &assertion, attr, f)))
        } else {
            Ok(JudgeResult {
                name: self.name.clone(),
                status: JudgeStatus::InternalError,
                exec_time_ms: 0,
            })
        }
    }

    pub fn in_path(&self, problem_id: &str) -> anyhow::Result<PathBuf> {
        Ok(AizuOnlineJudge::problem_dir_path(problem_id)?
            .join("in")
            .join(&self.name)
            .with_extension("in"))
    }

    pub fn out_path(&self, problem_id: &str) -> anyhow::Result<PathBuf> {
        Ok(AizuOnlineJudge::problem_dir_path(problem_id)?
            .join("out")
            .join(&self.name)
            .with_extension("out"))
    }
}
