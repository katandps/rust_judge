use crate::{
    attribute::VerifyAttribute,
    judge::{JudgeResult, JudgeStatus, VerifyResult},
};
use serde::{Deserialize, Serialize};
use std::{
    fs::{create_dir_all, File},
    io::{Read, Write},
    path::PathBuf,
    time::Duration,
};
use tokio::{runtime, time};

type SolveFunc = fn(&[u8], &mut Vec<u8>);

pub trait Service {
    fn fetch_testcases(problem_id: &str) -> anyhow::Result<()>;
    fn verify(attr: VerifyAttribute, f: SolveFunc) -> anyhow::Result<VerifyResult>;
    fn url(problem_id: &str) -> String;
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
    fn url(problem_id: &str) -> String {
        format!("https://onlinejudge.u-aizu.ac.jp/problems/{problem_id}")
    }

    fn fetch_testcases(problem_id: &str) -> anyhow::Result<()> {
        let mut problem_dir = crate::app_cache_directory();
        problem_dir.push("aizu_online_judge");
        problem_dir.push(problem_id);
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
            .map(|header| header.verify(attr, f))
            .collect();
        let success = cases
            .iter()
            .all(|result| result.status == JudgeStatus::Accepted);
        VerifyResult { success, cases }
    }
}

pub struct StaticAssertion;
impl StaticAssertion {
    pub fn assert(
        mut expect: impl std::io::Read,
        mut actual: impl std::io::Read,
    ) -> anyhow::Result<bool> {
        let (mut actual_values, mut expect_values) = (Vec::new(), Vec::new());
        {
            let mut buf = String::new();
            expect.read_to_string(&mut buf)?;
            for v in buf.split_ascii_whitespace() {
                expect_values.push(v.to_string());
            }
        }
        {
            let mut buf = String::new();
            actual.read_to_string(&mut buf)?;
            for v in buf.split_ascii_whitespace() {
                actual_values.push(v.to_string());
            }
        }
        if expect_values == actual_values {
            Ok(true)
        } else {
            log::error!("expect: {:?}\nactual: {:?}", expect_values, actual_values);
            Ok(false)
        }
    }
}
impl AOJTestCaseHeader {
    fn verify(&self, attr: &VerifyAttribute, f: SolveFunc) -> JudgeResult {
        let in_path = self.in_path(&attr.problem_id);
        let out_path = self.out_path(&attr.problem_id);
        if !in_path.exists() {
            log::warn!("in file is not found {}:{}", attr.problem_id, self.name);
        }
        if !out_path.exists() {
            log::warn!("out file is not found {}:{}", attr.problem_id, self.name);
        }
        if in_path.exists() && out_path.exists() {
            runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(self.verify_inner(in_path, out_path, attr, f))
        } else {
            JudgeResult {
                name: self.name.clone(),
                status: JudgeStatus::InternalError,
                exec_time_ms: 0,
            }
        }
    }

    async fn verify_inner(
        &self,
        in_path: PathBuf,
        out_path: PathBuf,
        attr: &VerifyAttribute,
        f: SolveFunc,
    ) -> JudgeResult {
        let mut ret = JudgeResult {
            name: self.name.clone(),
            status: JudgeStatus::InternalError,
            exec_time_ms: 0,
        };
        let Ok(in_buf) = read_file(&in_path) else {
            return ret;
        };
        let Ok(expect) = read_file(&out_path) else {
            return ret;
        };

        let run = async {
            let mut actual = Vec::new();
            let now = time::Instant::now();
            f(&in_buf, &mut actual);
            (actual, now.elapsed())
        };
        let sleep = time::sleep(Duration::from_millis(attr.time_limit_ms as u64));

        tokio::select! {
            _ = sleep => {
                // うまく動作していない 度を越えたTLEはこちらで打ち切りたい
                ret.status = JudgeStatus::TimeLimitExceeded
            },
            (actual, elapsed) = run => {
                ret.exec_time_ms = elapsed.as_millis() as u64;
                match StaticAssertion::assert(&expect[..], &actual[..]) {
                    Ok(status) => {
                        if status && ret.exec_time_ms <= attr.time_limit_ms {
                            ret.status = JudgeStatus::Accepted
                        } else if !status {
                            ret.status = JudgeStatus::WrongAnswer
                        } else {
                            ret.status = JudgeStatus::TimeLimitExceeded
                        }
                    }
                    Err(e) => {
                        log::error!("{:?}", e);
                        ret.status = JudgeStatus::InternalError
                    }
                }
            },

        }
        ret
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

fn read_file(path: &PathBuf) -> anyhow::Result<Vec<u8>> {
    let mut buf = Vec::new();
    File::open(&path)?.read_to_end(&mut buf)?;
    Ok(buf)
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
