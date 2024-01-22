use crate::{attribute::VerifyAttribute, judge::VerifyResult, Service, SolveFunc};

pub struct YukiCoder;

impl Service for YukiCoder {
    const SERVICE_NAME: &'static str = "yukicoder";
    fn url(problem_id: &str) -> String {
        format!("https://yukicoder.me/problems/no/{problem_id}")
    }
    fn fetch_testcases(_problem_id: &str) -> anyhow::Result<()> {
        todo!()
    }
    fn verify(_attr: VerifyAttribute, _f: SolveFunc) -> anyhow::Result<VerifyResult> {
        todo!()
    }
}
