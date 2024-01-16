use crate::{attribute::VerifyAttribute, judge::VerifyResult, Service};

pub struct LibraryChecker;

impl Service for LibraryChecker {
    const SERVICE_NAME: &'static str = "library_checker";
    fn fetch_testcases(problem_id: &str) -> anyhow::Result<()> {
        todo!()
    }
    fn url(problem_id: &str) -> String {
        format!("https://judge.yosupo.jp/problem/{problem_id}")
    }
    fn verify(attr: VerifyAttribute, f: crate::SolveFunc) -> anyhow::Result<VerifyResult> {
        todo!()
    }
}
