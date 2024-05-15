use reqwest::blocking;
use std::path::Path;
// use std::{fs::File, io::Write};
// use verify_core::service::atcoder::{header_path, AtCoderHeader};

fn _download_testcase_info(
    _client: blocking::Client,
    _problem_id: &str,
    _problem_dir: &Path,
) -> anyhow::Result<()> {
    // let in_url = format!("{BASE_URL}/{problem_id}/file/in");
    // let response = client
    //     .get(in_url)
    //     .header(
    //         "Authorization",
    //         get_session().with_context(|| "could not get session key")?,
    //     )
    //     .send()?;
    // let text = &response.text()?;
    // let list: Vec<String> = serde_json::from_str(text)?;
    // let header = AtCoderHeader {
    //     problem_id: problem_id.to_string(),
    //     list,
    // };

    // let header_path = header_path(problem_dir);
    // File::create(header_path)
    //     .expect("could not create header file")
    //     .write_all(serde_json::to_string(&header)?.as_bytes())?;
    Ok(())
}

const _BASE_URL: &str = "";
const _URL: &str = "https://www.dropbox.com/sh/arnpe0ef5wds8cv/AAAk_SECQ2Nc6SVGii3rHX6Fa?dl=0";
