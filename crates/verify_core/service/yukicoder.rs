use std::{
    fs::{create_dir_all, File},
    io::{Read, Write},
    path::PathBuf,
};

use anyhow::Error;
use reqwest::blocking;

use crate::{attribute::VerifyAttribute, judge::VerifyResult, Service, SolveFunc};

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
    fn verify(_attr: VerifyAttribute, _f: SolveFunc) -> anyhow::Result<VerifyResult> {
        todo!()
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
        header_path: PathBuf,
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
                let header_path = download_testcase_info(client, &problem_id, &problem_dir)?;
                Ok(Self::DownloadTestCases {
                    problem_id,
                    problem_dir,
                    header_path,
                })
            }
            Self::DownloadTestCases {
                problem_id,
                problem_dir,
                header_path,
            } => {
                let client = crate::blocking_client()?;
                download_testcases(client, &problem_id, &problem_dir, &header_path)?;
                Ok(Self::Done)
            }
            Self::Done => Err(Error::msg("Task is completed.")),
        }
    }
}
fn create_problem_directory(problem_id: &str, base_dir: &PathBuf) -> anyhow::Result<PathBuf> {
    let mut problem_dir = base_dir.clone();
    problem_dir.push("yukicoder");
    problem_dir.push(&problem_id);
    let in_dir = problem_dir.join("in");
    let out_dir = problem_dir.join("out");
    create_dir_all(&in_dir)?;
    create_dir_all(&out_dir)?;

    Ok(problem_dir)
}

fn download_testcase_info(
    client: blocking::Client,
    problem_id: &str,
    problem_dir: &PathBuf,
) -> anyhow::Result<PathBuf> {
    let in_url = format!("{BASE_URL}/{problem_id}/file/in");
    let response = client
        .get(in_url)
        .header("Authorization", get_session()?)
        .send()?;
    let text = &response.text()?;
    dbg!(&text);
    let list: Vec<String> = serde_json::from_str(&text)?;
    let header_path = problem_dir.join("header.json");
    File::create(&header_path)?.write_all(serde_json::to_string(&list)?.as_bytes())?;
    Ok(header_path)
}

fn download_testcases(
    client: blocking::Client,
    problem_id: &str,
    problem_dir: &PathBuf,
    header_path: &PathBuf,
) -> anyhow::Result<()> {
    let mut buf = Vec::new();
    File::open(header_path)?.read_to_end(&mut buf)?;
    let list: Vec<String> = serde_json::from_slice(&buf)?;
    for target in list {
        let in_url = format!("{BASE_URL}/{problem_id}/file/in/{target}");
        let response = client
            .get(in_url)
            .header("Authorization", get_session()?)
            .send()?;
        let text = response.text()?;
        let in_path = problem_dir.join("in").join(&target);
        File::create(&in_path)?.write_all(&text.as_bytes())?;

        let out_url = format!("{BASE_URL}/{problem_id}/file/out/{target}");
        let response = client
            .get(out_url)
            .header("Authorization", get_session()?)
            .send()?;
        let text = response.text()?;
        let out_path = problem_dir.join("out").join(&target);
        File::create(&out_path)?.write_all(&text.as_bytes())?;
    }
    Ok(())
}

fn get_session() -> anyhow::Result<String> {
    Ok(format!("bearer {}", std::env::var("YUKICODER_TOKEN")?))
}
