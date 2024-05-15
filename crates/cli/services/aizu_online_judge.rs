use std::{
    fs::{create_dir_all, File},
    io::Write,
    time::Duration,
};

use verify_core::{
    service::aizu_online_judge::{AOJTestCaseHeaders, AizuOnlineJudge},
    ProblemForVerify,
};

pub fn fetch_testcases(problem: &ProblemForVerify) -> anyhow::Result<()> {
    let problem_id = &problem.problem_id;
    let mut problem_dir = verify_core::app_cache_directory();
    problem_dir.push("aizu_online_judge");
    problem_dir.push(problem_id);
    if !problem_dir.exists() {
        create_dir_all(&problem_dir)?;
    }
    let in_dir = problem_dir.join("in");
    let out_dir = problem_dir.join("out");
    create_dir_all(in_dir)?;
    create_dir_all(out_dir)?;

    let url = format!("https://judgedat.u-aizu.ac.jp/testcases/{problem_id}/header");
    let client = super::blocking_client()?;

    let headers: AOJTestCaseHeaders = client
        .get(url)
        .timeout(Duration::from_secs(5))
        .send()?
        .json()?;
    File::create(AizuOnlineJudge::header_path(&problem_id)?)?
        .write_all(serde_json::to_string(&headers)?.as_bytes())?;

    for header in headers.headers {
        let serial = header.serial;
        let in_path = header.in_path(&problem_id)?;
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
        let out_path = header.out_path(&problem_id)?;
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
