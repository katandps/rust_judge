use anyhow::{Context, Error};
use reqwest::blocking;
use serde::{Deserialize, Serialize};
use std::{
    fs::{create_dir_all, File},
    io::{Read, Write},
    path::{Path, PathBuf},
};
use tokio::runtime;

use crate::{
    attribute::VerifyAttribute,
    judge::{self, JudgeResult, JudgeStatus, StaticAssertion, VerifyResult},
    Service, SolveFunc,
};

pub struct Yukicoder;
const BASE_URL: &str = "https://yukicoder.me/api/v1/problems/no";

impl Service for Yukicoder {
    const SERVICE_NAME: &'static str = "yukicoder";
    fn url(problem_id: &str) -> String {
        format!("https://yukicoder.me/problems/no/{problem_id}")
    }
    fn fetch_testcases(problem_id: &str) -> anyhow::Result<()> {
        let mut task = YukicoderTask::new(problem_id);
        while task != YukicoderTask::Done {
            task = task.run()?;
        }
        Ok(())
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
struct YukicoderHeader {
    problem_id: String,
    list: Vec<String>,
}
impl YukicoderHeader {
    fn from_file(path: &PathBuf) -> Self {
        let mut buf = Vec::new();
        File::open(path)
            .expect("header file is not found")
            .read_to_end(&mut buf)
            .expect("could not load file");
        serde_json::from_slice(&buf).expect("saved header file is invalid")
    }

    fn download(
        &self,
        problem_id: &str,
        client: blocking::Client,
        problem_dir: &Path,
    ) -> anyhow::Result<()> {
        for target in &self.list {
            let in_url = format!("{BASE_URL}/{problem_id}/file/in/{target}");
            let response = client
                .get(in_url)
                .header("Authorization", get_session()?)
                .send()?;
            let text = response.text()?;
            let in_path = problem_dir.join("in").join(target);
            File::create(&in_path)?.write_all(text.as_bytes())?;

            let out_url = format!("{BASE_URL}/{problem_id}/file/out/{target}");
            let response = client
                .get(out_url)
                .header("Authorization", get_session()?)
                .send()?;
            let text = response.text()?;
            let out_path = problem_dir.join("out").join(target);
            File::create(&out_path)?.write_all(text.as_bytes())?;
        }
        Ok(())
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

#[derive(Clone, Debug, PartialEq)]
enum YukicoderTask {
    CreateProblemDirectory {
        problem_id: String,
        base_dir: PathBuf,
    },
    DownloadTestCaseInfo {
        problem_id: String,
        problem_dir: PathBuf,
    },
    DownloadTestCases {
        problem_id: String,
        problem_dir: PathBuf,
    },
    Done,
}
impl YukicoderTask {
    fn new(problem_id: &str) -> Self {
        Self::CreateProblemDirectory {
            problem_id: problem_id.into(),
            base_dir: crate::app_cache_directory(),
        }
    }
    fn run(self) -> anyhow::Result<Self> {
        match self {
            Self::CreateProblemDirectory {
                problem_id,
                base_dir,
            } => {
                let problem_dir = create_problem_directory(&problem_id, &base_dir)?;
                Ok(Self::DownloadTestCaseInfo {
                    problem_id,
                    problem_dir,
                })
            }
            Self::DownloadTestCaseInfo {
                problem_id,
                problem_dir,
            } => {
                let client = crate::blocking_client()?;
                download_testcase_info(client, &problem_id, &problem_dir)?;
                Ok(Self::DownloadTestCases {
                    problem_id,
                    problem_dir,
                })
            }
            Self::DownloadTestCases {
                problem_id,
                problem_dir,
            } => {
                let client = crate::blocking_client()?;
                let path = header_path(&problem_dir);
                let testcases = YukicoderHeader::from_file(&path);
                testcases.download(&problem_id, client, &problem_dir)?;
                Ok(Self::Done)
            }
            Self::Done => Err(Error::msg("Task is completed.")),
        }
    }
}
fn create_problem_directory(problem_id: &str, base_dir: &Path) -> anyhow::Result<PathBuf> {
    let mut problem_dir = base_dir.to_path_buf();
    problem_dir.push("yukicoder");
    problem_dir.push(problem_id);
    let in_dir = problem_dir.join("in");
    let out_dir = problem_dir.join("out");
    create_dir_all(in_dir).with_context(|| "could not create in directory")?;
    create_dir_all(out_dir).with_context(|| "could not create out directory")?;

    Ok(problem_dir)
}

fn download_testcase_info(
    client: blocking::Client,
    problem_id: &str,
    problem_dir: &Path,
) -> anyhow::Result<()> {
    let in_url = format!("{BASE_URL}/{problem_id}/file/in");
    let response = client
        .get(in_url)
        .header(
            "Authorization",
            get_session().with_context(|| "could not get session key")?,
        )
        .send()?;
    let text = &response.text()?;
    let list: Vec<String> = serde_json::from_str(text)?;
    let header = YukicoderHeader {
        problem_id: problem_id.to_string(),
        list,
    };

    let header_path = header_path(problem_dir);
    File::create(header_path)
        .expect("could not create header file")
        .write_all(serde_json::to_string(&header)?.as_bytes())?;
    Ok(())
}

fn header_path(problem_dir: &Path) -> PathBuf {
    problem_dir.join("header").with_extension("json")
}

fn get_session() -> anyhow::Result<String> {
    Ok(format!("bearer {}", std::env::var("YUKICODER_TOKEN")?))
}
