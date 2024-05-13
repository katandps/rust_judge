use anyhow::anyhow;
use dropbox_sdk::client_trait;
use dropbox_sdk::default_client::{NoauthDefaultClient, UserAuthDefaultClient};
use dropbox_sdk::files;
use dropbox_sdk::files::SharedLink;
use dropbox_sdk::oauth2::{Authorization, AuthorizeUrlBuilder, Oauth2Type};
use serde::Deserialize;
use std::io;
use std::io::Write;

pub fn setup() -> anyhow::Result<()> {
    let config = envy::from_env::<DropboxSettings>()?;
    let flow_type = Oauth2Type::AuthorizationCode {
        client_secret: config.dropbox_app_secret,
    };
    let authorization_url = AuthorizeUrlBuilder::new(&config.dropbox_app_key, &flow_type).build();

    fn prompt(msg: &str) -> anyhow::Result<String> {
        eprint!("{}: ", msg);
        io::stderr().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input.trim().to_owned())
    }
    let auth_code = prompt(&format!(
        "以下のURLを開いて得たcodeを入力してください\n{}\n",
        &authorization_url
    ))?;
    let auth = Authorization::from_auth_code(config.dropbox_app_key, flow_type, auth_code, None);

    let token = auth
        .save()
        .ok_or(anyhow!("refresh tokenが取得できませんでした\n"))?;
    eprintln!(
        "以下のコードをDROPBOX_REFRESH_TOKENとして設定してください\n{}",
        token[2..].to_owned()
    );
    Ok(())
}

pub fn list() -> anyhow::Result<()> {
    let config = envy::from_env::<DropboxSettings>()?;
    let refresh_token = config
        .dropbox_refresh_token
        .ok_or(anyhow!("DROBOX_REFRESH_TOKEN is not set"))?;
    let mut auth = Authorization::from_client_secret_refresh_token(
        config.dropbox_app_key,
        config.dropbox_app_secret,
        refresh_token,
    );
    let p = auth.obtain_access_token(NoauthDefaultClient::default());
    let client = UserAuthDefaultClient::new(auth);
    let mut problem_list = Vec::new();
    list_folder(&client, &mut problem_list)?;
    problem_list.sort();
    dbg!(&problem_list, problem_list.len());
    Ok(())
}

fn list_folder(
    client: &impl client_trait::UserAuthClient,
    list: &mut Vec<String>,
) -> anyhow::Result<()> {
    let result = files::list_folder(
        client,
        &files::ListFolderArg::new(String::new()).with_shared_link(SharedLink::new(
            ATCODER_TESTCASE_DROPBOX_SHARED_URL.to_string(),
        )),
    )??;
    read_folder_result(result, client, list)
}
fn list_folder_continue(
    cursor: String,
    client: &impl client_trait::UserAuthClient,
    list: &mut Vec<String>,
) -> anyhow::Result<()> {
    match files::list_folder_continue(client, &files::ListFolderContinueArg::new(cursor)) {
        Ok(Ok(result)) => read_folder_result(result, client, list),
        Ok(Err(e)) => panic!("failed to list folder: {e}"),
        Err(e) => panic!("failed to request api: {e}"),
    }
}
const ATCODER_TESTCASE_DROPBOX_SHARED_URL: &str =
    "https://www.dropbox.com/sh/arnpe0ef5wds8cv/AAAk_SECQ2Nc6SVGii3rHX6Fa?dl=0";

fn read_folder_result(
    result: files::ListFolderResult,
    client: &impl client_trait::UserAuthClient,
    list: &mut Vec<String>,
) -> anyhow::Result<()> {
    for entry in result.entries {
        match entry {
            files::Metadata::Folder(folder_meta_data) => list.push(folder_meta_data.name),
            _ => (),
        }
    }
    if result.has_more {
        list_folder_continue(result.cursor, client, list)?;
    }
    Ok(())
}

#[derive(Deserialize, Debug)]
struct DropboxSettings {
    dropbox_app_key: String,
    dropbox_app_secret: String,
    dropbox_refresh_token: Option<String>,
}
