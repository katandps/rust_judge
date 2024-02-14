use crate::{judge::VerifyResult, Service};

pub struct AtCoder;
impl Service for AtCoder {
    const SERVICE_NAME: &'static str = "atcoder";
    fn url(_problem_id: &str) -> String {
        format!("https://atcoder.jp/")
    }
    fn fetch_testcases(_problem_id: &str) -> anyhow::Result<()> {
        Ok(())
    }
    fn verify(
        _attr: crate::attribute::VerifyAttribute,
        _f: crate::SolveFunc,
    ) -> anyhow::Result<crate::judge::VerifyResult> {
        Ok(VerifyResult { cases: Vec::new() })
    }
}
