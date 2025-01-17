use crate::{
    attribute::VerifyAttribute,
    judge::{Assertion, CheckBinaryAssertion, JudgeResult, JudgeStatus, VerifyResult},
    Service, SolveFunc,
};
use serde::{Deserialize, Serialize};
use std::{
    fs::{read_dir, read_to_string},
    path::{Path, PathBuf},
    time::Duration,
};
use tokio::{runtime, time};

pub struct LibraryChecker;

impl Service for LibraryChecker {
    const SERVICE_NAME: &'static str = "library_checker";

    fn url(problem_id: &str) -> String {
        format!("https://judge.yosupo.jp/problem/{problem_id}")
    }
    fn verify(attr: VerifyAttribute, f: crate::SolveFunc) -> anyhow::Result<VerifyResult> {
        let problem = find_problem(&attr.problem_id)?;
        Ok(problem.verify(&attr, f))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Problem {
    pub dir: PathBuf,
    info: ProblemInfo,
}

impl Problem {
    fn verify(&self, attr: &VerifyAttribute, f: SolveFunc) -> VerifyResult {
        let cases = self
            .info
            .tests
            .iter()
            .flat_map(|c| c.verify(&self.dir, attr, f))
            .collect();
        VerifyResult { cases }
    }
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

impl TestCase {
    fn verify(&self, root_dir: &Path, attr: &VerifyAttribute, f: SolveFunc) -> Vec<JudgeResult> {
        fn case_file_name(name: &str, i: usize) -> String {
            let mut iter = name.rsplitn(2, '.');
            let after = iter.next();
            let before = iter.next();
            if before == Some("") {
                Some(name)
            } else {
                before.or(after)
            }
            .map(|name| format!("{}_{:02}", name, i))
            .unwrap()
        }
        let in_dir = root_dir.join("in");
        let out_dir = root_dir.join("out");

        let mut ret = Vec::new();
        for i in 0..self.number {
            let name = case_file_name(&self.name, i);
            let in_path = in_dir.join(&name).with_extension("in");
            let out_path = out_dir.join(&name).with_extension("out");
            if !in_path.exists() {
                println!("in file is not found: {}", &in_path.to_string_lossy());
            }
            if !out_path.exists() {
                println!("out file is not found: {}", &out_path.to_string_lossy());
            }
            let assertion = CheckBinaryAssertion {
                input_path: in_path.clone(),
                expect_path: out_path.clone(),
                checker_path: root_dir.join("checker"),
            };
            if in_path.exists() && out_path.exists() {
                ret.push(
                    runtime::Builder::new_current_thread()
                        .enable_all()
                        .build()
                        .unwrap()
                        .block_on(self.verify_inner(name, &assertion, attr, f)),
                )
            } else {
                ret.push(JudgeResult {
                    name,
                    status: JudgeStatus::InternalError,
                    exec_time_ms: 0,
                });
            }
        }
        ret
    }

    async fn verify_inner(
        &self,
        name: String,
        assertion: &CheckBinaryAssertion,
        attr: &VerifyAttribute,
        f: SolveFunc,
    ) -> JudgeResult {
        let mut ret = JudgeResult {
            name,
            status: JudgeStatus::InternalError,
            exec_time_ms: 0,
        };
        let Ok(in_buf) = crate::read_file(&assertion.input_path) else {
            return ret;
        };

        let run = async {
            let now = time::Instant::now();
            let actual = ::std::panic::catch_unwind(|| {
                let mut actual = Vec::new();
                f(&in_buf, &mut actual);
                actual
            });
            (actual, now.elapsed())
        };
        let sleep = time::sleep(Duration::from_millis(attr.time_limit_ms));
        tokio::select! {
            _ = sleep => {
                // うまく動作していない 度を越えたTLEはこちらで打ち切りたい
                ret.status = JudgeStatus::TimeLimitExceeded
            },
            (actual, elapsed) = run => {
                ret.exec_time_ms = elapsed.as_millis() as u64;
                if let Ok(actual) = actual {
                    match assertion.assert(&String::from_utf8_lossy(&actual)) {
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
                            println!("{:?}", e);
                            ret.status = JudgeStatus::InternalError
                        }
                    }
                } else {
                    ret.status = JudgeStatus::RuntimeError
                }
            },

        }
        ret
    }
}

pub fn find_problem(problem_id: &str) -> anyhow::Result<Problem> {
    for entry in read_dir(root_dir()?)?.flatten() {
        let mut path = entry.path().join(problem_id).join("info.toml");
        if path.is_file() {
            log::debug!("found problem: {}", path.display());
            let data = read_to_string(&path)?;
            let info: ProblemInfo = toml::from_str(&data)?;
            path.pop();
            return Ok(Problem { dir: path, info });
        }
    }
    Err(anyhow::format_err!("info.toml is not found."))
}

pub fn root_dir() -> anyhow::Result<PathBuf> {
    Ok(crate::app_cache_directory().join("library_checker"))
}
