use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context};
use reqwest::blocking;
use verify_core::service::yukicoder::{create_problem_directory, header_path, YukicoderHeader};
use verify_core::ProblemForVerify;

pub fn fetch_testcases(problem: &ProblemForVerify) -> anyhow::Result<()> {
    let mut task = YukicoderTask::new(&problem.problem_id);
    while task != YukicoderTask::Done {
        task = task.run()?;
    }
    Ok(())
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
            base_dir: verify_core::app_cache_directory(),
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
                let client = super::blocking_client()?;
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
                let client = super::blocking_client()?;
                let path = header_path(&problem_dir);
                let testcases = YukicoderHeader::from_file(&path);
                download(&testcases, &problem_id, client, &problem_dir)?;
                Ok(Self::Done)
            }
            Self::Done => Err(anyhow!("Task is completed.")),
        }
    }
}
const BASE_URL: &str = "https://yukicoder.me/api/v1/problems/no";

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

fn download(
    header: &YukicoderHeader,
    problem_id: &str,
    client: blocking::Client,
    problem_dir: &Path,
) -> anyhow::Result<()> {
    for target in &header.list {
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

fn get_session() -> anyhow::Result<String> {
    Ok(format!("bearer {}", std::env::var("YUKICODER_TOKEN")?))
}
