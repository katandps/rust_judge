use dirs::cache_dir;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use serde::Deserialize;
use std::fs::File;
use std::io::Write;
use std::{env::temp_dir, fs::create_dir_all, path::PathBuf, time::Duration};

use crate::attribute::VerifyAttribute;

pub trait Service {
    fn build(attr: VerifyAttribute, f: Ident) -> TokenStream;
    fn get_testcases(problem_id: String) -> anyhow::Result<()>;
    fn run(f: Ident) -> TokenStream {
        quote! {
            let case = "12345".as_bytes();
            let mut buf = Vec::new();
            #f(case, &mut buf);
        }
    }
    const SERVICE_NAME: &'static str;
}
pub fn save_verify_info<S: Service>(attr: &VerifyAttribute) -> anyhow::Result<()> {
    let mut target = PathBuf::from(crate::target_directory()?);
    target.push(crate::APP_NAME);
    target.push(S::SERVICE_NAME);
    create_dir_all(&target)?;
    target.push(&attr.problem_id);
    let mut file = File::create(&target)?;
    file.flush()?;
    file.write_all(serde_json::to_string(&attr)?.as_bytes())?;
    Ok(())
}

#[derive(Deserialize, Debug)]
struct AOJTestCaseHeaders {
    headers: Vec<AOJTestCaseHeader>,
}

#[derive(Deserialize, Debug)]
struct AOJTestCaseHeader {
    serial: u32,
    name: String,
}

pub struct AizuOnlineJudge;

impl Service for AizuOnlineJudge {
    fn get_testcases(problem_id: String) -> anyhow::Result<()> {
        let mut problem_dir = app_cache_directory();
        problem_dir.push("aizu_online_judge");
        problem_dir.push(&problem_id);
        if !problem_dir.exists() {
            create_dir_all(&problem_dir).ok();
        }
        let url = format!("https://judgedat.u-aizu.ac.jp/testcases/{problem_id}/header");
        let client = blocking_client()?;
        let headers: AOJTestCaseHeaders = client
            .get(url)
            .timeout(Duration::from_secs(5))
            .send()?
            .json()?;
        dbg!(headers);
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
fn app_cache_directory() -> PathBuf {
    let mut path = cache_dir().unwrap_or_else(temp_dir);
    path.push(crate::APP_NAME);
    path
}
fn blocking_client() -> reqwest::Result<reqwest::blocking::Client> {
    reqwest::blocking::Client::builder().build()
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
