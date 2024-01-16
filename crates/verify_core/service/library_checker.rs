use crate::{attribute::VerifyAttribute, judge::VerifyResult, Service};
use serde::{Deserialize, Serialize};
use std::{
    fs::{create_dir_all, read_dir, read_to_string},
    path::PathBuf,
    process::Command,
};

pub struct LibraryChecker;

impl Service for LibraryChecker {
    const SERVICE_NAME: &'static str = "library_checker";
    fn fetch_testcases(problem_id: &str) -> anyhow::Result<()> {
        fetch_problem_repository()?;
        let problem = find_problem(problem_id)?;
        let in_dir = problem.dir.join("in");
        let out_dir = problem.dir.join("out");
        create_dir_all(&in_dir)?;
        create_dir_all(&out_dir)?;

        Command::new(option_env!("PYTHON").unwrap_or("python"))
            .arg(root_dir().join("generate.py"))
            .arg(problem.dir.join("info.toml"))
            .output()?;
        Ok(())
    }
    fn url(problem_id: &str) -> String {
        format!("https://judge.yosupo.jp/problem/{problem_id}")
    }
    fn verify(attr: VerifyAttribute, f: crate::SolveFunc) -> anyhow::Result<VerifyResult> {
        let problem = find_problem(&attr.problem_id)?;
        let in_dir = problem.dir.join("in");
        let out_dir = problem.dir.join("out");
        for case in &problem.info.tests {
            for i in 0..case.number {
                let name = format!("{}{i}", case.name);
                let input = in_dir.join(&name).with_extension("in");
                let output = out_dir.join(&name).with_extension("out");
            }
        }

        todo!()
    }
}

fn root_dir() -> PathBuf {
    crate::app_cache_directory().join("library_checker")
}
const LIBRARY_CHECKER_GIT_REPOSITORY: &str = "https://github.com/yosupo06/library-checker-problems";
fn fetch_problem_repository() -> anyhow::Result<()> {
    let root_dir = root_dir();
    if root_dir.exists() {
        Command::new("git")
            .arg("-C")
            .arg(root_dir.as_os_str())
            .arg("pull")
            .output()?;
    } else {
        Command::new("git")
            .arg("clone")
            .arg(LIBRARY_CHECKER_GIT_REPOSITORY)
            .arg(root_dir.as_os_str())
            .output()?;
    }
    Ok(())
}
#[derive(Debug, Clone, Deserialize, Serialize)]
struct Problem {
    dir: PathBuf,
    info: ProblemInfo,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
struct ProblemInfo {
    tests: Vec<TestCase>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct TestCase {
    name: String,
    number: usize,
}

fn find_problem(problem_id: &str) -> anyhow::Result<Problem> {
    let root_dir = crate::app_cache_directory().join("library_checker");
    for entry in read_dir(root_dir)?.flatten() {
        let mut path = entry.path().join(problem_id).join("info.toml");
        if path.is_file() {
            let data = read_to_string(&path)?;
            let info: ProblemInfo = toml::from_str(&data)?;
            path.pop();
            return Ok(Problem { dir: path, info });
        }
    }
    panic!()
}
