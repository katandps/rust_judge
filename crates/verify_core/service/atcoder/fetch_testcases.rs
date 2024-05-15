use anyhow::Context;
use reqwest::blocking;
use std::path::Path;
use std::{fs::File, io::Write};

use super::{get_session, header_path, AtCoderHeader};

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
    let header = AtCoderHeader {
        problem_id: problem_id.to_string(),
        list,
    };

    let header_path = header_path(problem_dir);
    File::create(header_path)
        .expect("could not create header file")
        .write_all(serde_json::to_string(&header)?.as_bytes())?;
    Ok(())
}

const BASE_URL: &str = "";
