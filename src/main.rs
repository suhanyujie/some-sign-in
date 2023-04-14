use anyhow::{Result as AnyResult, Ok};
use reqwest::{header::{HeaderMap, self}};
use serde::{Deserialize, Serialize};

/// 启动一个定时任务服务
/// 每天执行签到请求 todo


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

async fn req_sign_in() ->AnyResult<()> {
    let config = read_config()?;
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
    println!("{:?}", res_obj);
    Ok(())
}

async fn post(url: &str) ->AnyResult<String> {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    let resp = client.post(url).headers(headers).send().await?.bytes().await?;
    let resp_str = String::from_utf8_lossy(&resp);
    Ok(resp_str.to_string())
}

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    user: ConfigUser,
    sys: ConfigSys,
}

#[derive(Debug, Deserialize, Serialize)]
struct ConfigUser {
    cookie: String,
    sign_req_body: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct ConfigSys {
    sign_url: String,
}

fn read_config() ->AnyResult<Config>{
    let data = std::fs::read_to_string("./env.local.toml")?;
    let obj: Config = toml::from_str(&data)?;
    Ok(obj)
}

#[tokio::main]
async fn main() {
    match req_sign_in().await {
        std::result::Result::Ok(()) =>{
            println!("ok...");
        },
        std::result::Result::Err(err) =>{
            eprintln!("error: {:#?}", err);
        }
    }
}
