use proc_macro2::{Ident, TokenStream};
use quote::quote;
use serde::{Deserialize, Serialize};
use std::{
    fs::{create_dir_all, File},
    io::{Read, Write},
    path::PathBuf,
    time::Duration,
};

use crate::{
    attribute::VerifyAttribute,
    judge::{JudgeResult, VerifyResult},
};

pub trait Service {
    fn build(attr: VerifyAttribute, f: Ident) -> TokenStream;
    fn fetch_testcases(problem_id: String) -> anyhow::Result<()>;
    fn run(f: Ident) -> TokenStream {
        quote! {
            let case = "12345".as_bytes();
            let mut buf = Vec::new();
            #f(case, &mut buf);
        }
    }
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
    fn fetch_testcases(problem_id: String) -> anyhow::Result<()> {
        let mut problem_dir = crate::app_cache_directory();
        problem_dir.push("aizu_online_judge");
        problem_dir.push(&problem_id);
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
            let in_path = in_dir.join(&header.name).with_extension("in");
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
            let out_path = out_dir.join(&header.name).with_extension("out");
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

    fn build(_attr: VerifyAttribute, f: Ident) -> TokenStream {
        // todo read attr file and verify with cache;
        // dbg!(problem_id, eps, tl, &f);
        quote! {
            let case = "12345".as_bytes();
            let mut buf = Vec::new();
            #f(case, &mut buf);
        }
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

    pub fn verify(attr: VerifyAttribute) -> anyhow::Result<VerifyResult> {
        let mut buf = Vec::new();
        File::open(Self::header_path(&attr.problem_id))?.read_to_end(&mut buf)?;
        let headers: AOJTestCaseHeaders = serde_json::from_slice(&buf)?;
        let cases: Vec<_> = headers
            .headers
            .iter()
            .map(|_header| {
                // todo verify
                JudgeResult::Accepted
            })
            .collect();
        let success = cases.iter().all(|result| result == &JudgeResult::Accepted);
        Ok(VerifyResult { success, cases })
    }
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
