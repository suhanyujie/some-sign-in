use anyhow::{Ok, Result as AnyResult};
use reqwest::header::{self};
use crate::conf::conf;
use serde::{Deserialize, Serialize};

/// glodos 签到
#[derive(Debug, Deserialize, Serialize)]
struct BaseResp {
    code: i32,
    message: String,
    list: Option<Vec<DailySignInfo>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct DailySignInfo {
    asset: String,
    balance: String,
    business: String,
    change: String,
    detail: String,
    id: i32,
    time: i64,
    user_id: i32,
}

pub async fn req_sign_in() -> AnyResult<()> {
    let config = conf::read_config()?;
    let sign_url = config.sys.sign_url;
    let client = reqwest::Client::new();
    let resp = client
        .post(sign_url)
        .header(header::COOKIE, config.user.cookie)
        .header(header::CONTENT_TYPE, "application/json; charset=utf-8")
        .body(config.user.sign_req_body)
        .send()
        .await?
        .text()
        .await?;
    let res_obj: Result<BaseResp, serde_json::Error> = serde_json::from_str(resp.as_str());
    if res_obj.is_ok() {
        println!("sign info list len {:?}", res_obj.unwrap().list.unwrap().len());
    } else {
        eprintln!("sign err {:?}", res_obj.err());
    }
    Ok(())
}