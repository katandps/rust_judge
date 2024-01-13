use serde::{Deserialize, Serialize};
use std::{
    fs::{create_dir_all, File},
    io::{Read, Write},
    path::PathBuf,
    time::Duration,
};

use crate::{
    attribute::VerifyAttribute,
    judge::{JudgeResult, VerifyResult},
};

type SolveFunc = fn(&[u8], &mut [u8]);

pub trait Service {
    fn save_verify_info(attr: &VerifyAttribute) -> anyhow::Result<()>;
    fn fetch_testcases(problem_id: String) -> anyhow::Result<()>;
    fn verify(attr: VerifyAttribute, f: SolveFunc) -> anyhow::Result<VerifyResult>;
    const SERVICE_NAME: &'static str;
}

#[derive(Deserialize, Serialize, Debug)]
struct AOJTestCaseHeaders {
    // problemId: String,
    headers: Vec<AOJTestCaseHeader>,
}

#[derive(Deserialize, Serialize, Debug)]
struct AOJTestCaseHeader {
    serial: u32,
    name: String,
    // inputSize: i64,
    // outputSize:i64,
    // score: i64,
}

pub struct AizuOnlineJudge;

impl Service for AizuOnlineJudge {
    fn save_verify_info(attr: &VerifyAttribute) -> anyhow::Result<()> {
        let mut target = PathBuf::from(crate::target_directory()?);
        target.push(crate::APP_NAME);
        target.push(Self::SERVICE_NAME);
        create_dir_all(&target)?;
        target.push(&attr.problem_id);
        let mut file = File::create(&target)?;
        file.flush()?;
        file.write_all(serde_json::to_string(&attr)?.as_bytes())?;
        Ok(())
    }

    fn fetch_testcases(problem_id: String) -> anyhow::Result<()> {
        let mut problem_dir = crate::app_cache_directory();
        problem_dir.push("aizu_online_judge");
        problem_dir.push(&problem_id);
        if !problem_dir.exists() {
            create_dir_all(&problem_dir)?;
        }
        let in_dir = problem_dir.join("in");
        let out_dir = problem_dir.join("out");
        create_dir_all(&in_dir)?;
        create_dir_all(&out_dir)?;

        let url = format!("https://judgedat.u-aizu.ac.jp/testcases/{problem_id}/header");
        let client = crate::blocking_client()?;

        let headers: AOJTestCaseHeaders = client
            .get(url)
            .timeout(Duration::from_secs(5))
            .send()?
            .json()?;
        File::create(Self::header_path(&problem_id))?
            .write_all(serde_json::to_string(&headers)?.as_bytes())?;

        for header in headers.headers {
            let serial = header.serial;
            let in_path = header.in_path(&problem_id);
            if !in_path.exists() {
                let in_url =
                    format!("https://judgedat.u-aizu.ac.jp/testcases/{problem_id}/{serial}/in");
                let bytes = client
                    .get(in_url)
                    .timeout(Duration::from_secs(5))
                    .send()?
                    .bytes()?;
                File::create(in_path)?.write_all(&bytes)?;
            }
            let out_path = header.out_path(&problem_id);
            if !out_path.exists() {
                let out_url =
                    format!("https://judgedat.u-aizu.ac.jp/testcases/{problem_id}/{serial}/out");
                let bytes = client
                    .get(out_url)
                    .timeout(Duration::from_secs(5))
                    .send()?
                    .bytes()?;
                File::create(out_path)?.write_all(&bytes)?;
            }
        }
        Ok(())
    }
    fn verify(attr: VerifyAttribute, f: SolveFunc) -> anyhow::Result<VerifyResult> {
        let mut buf = Vec::new();
        dbg!(&Self::header_path(&attr.problem_id));
        File::open(Self::header_path(&attr.problem_id))?.read_to_end(&mut buf)?;
        let headers: AOJTestCaseHeaders = serde_json::from_slice(&buf)?;
        Ok(headers.verify(&attr, f))
    }
    const SERVICE_NAME: &'static str = "aizu_online_judge";
}

impl AizuOnlineJudge {
    fn problem_dir_path(problem_id: &str) -> PathBuf {
        let mut problem_dir = crate::app_cache_directory();
        problem_dir.push("aizu_online_judge");
        problem_dir.push(problem_id);
        problem_dir
    }
    fn header_path(problem_id: &str) -> PathBuf {
        Self::problem_dir_path(problem_id)
            .join("header")
            .with_extension("json")
    }
}

impl AOJTestCaseHeaders {
    fn verify(&self, attr: &VerifyAttribute, f: SolveFunc) -> VerifyResult {
        let cases: Vec<_> = self
            .headers
            .iter()
            .map(|header| header.verify(attr, f).unwrap_or(JudgeResult::InternalError))
            .collect();
        let success = cases.iter().all(|result| result == &JudgeResult::Accepted);
        VerifyResult { success, cases }
    }
}

impl AOJTestCaseHeader {
    fn verify(&self, attr: &VerifyAttribute, f: SolveFunc) -> anyhow::Result<JudgeResult> {
        let in_path = self.in_path(&attr.problem_id);
        let out_path = self.out_path(&attr.problem_id);
        if in_path.exists() && out_path.exists() {
            let (mut in_buf, mut out_buf) = (Vec::new(), Vec::new());
            File::open(&in_path)?.read_to_end(&mut in_buf)?;
            File::open(&out_path)?.read_to_end(&mut out_buf)?;
            let mut actual = Vec::new();
            f(&in_buf, &mut actual);
            return if out_buf == actual {
                Ok(JudgeResult::Accepted)
            } else {
                Ok(JudgeResult::WrongAnswer)
            };
        }
        if !in_path.exists() {
            log::warn!("in file is not found {}:{}", attr.problem_id, self.name);
        }
        if !out_path.exists() {
            log::warn!("out file is not found {}:{}", attr.problem_id, self.name);
        }
        Ok(JudgeResult::InternalError)
    }
    fn in_path(&self, problem_id: &str) -> PathBuf {
        AizuOnlineJudge::problem_dir_path(problem_id)
            .join("in")
            .join(&self.name)
            .with_extension("in")
    }
    fn out_path(&self, problem_id: &str) -> PathBuf {
        AizuOnlineJudge::problem_dir_path(problem_id)
            .join("out")
            .join(&self.name)
            .with_extension("out")
    }
}

// pub struct LibraryChecker;

// impl LibraryChecker {
//     fn download_testcases(_problem_id: &str) {
//         todo!()
//     }
//     fn verify(_attr: VerifyAttribute, _f: &dyn Fn() -> ()) -> VerifyResult {
//         todo!()
//     }
// }
// pub struct AtCoder;

// impl Service for AtCoder {
//     fn download_testcases(_problem_id: &str) {
//         todo!()
//     }
//     fn verify(_attr: VerifyAttribute, _f: &dyn Fn() -> ()) -> VerifyResult {
//         todo!()
//     }
// }
// pub struct YukiCoder;

// impl Service for YukiCoder {
//     fn download_testcases(_problem_id: &str) {
//         todo!()
//     }
//     fn verify(_attr: VerifyAttribute, _f: &dyn Fn() -> ()) -> VerifyResult {
//         todo!()
//     }
// }
