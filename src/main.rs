use anyhow::{Result as AnyResult, Ok};
use reqwest::header::{HeaderMap, self};

/// 启动一个定时任务服务
/// 每天执行签到请求 todo


#[tokio::main]
async fn main() {
    req_sign_in().await;
}

async fn req_sign_in() ->AnyResult<()> {
    let sign_url = "https://glados.rocks/api/user/checkin";
    let resp = reqwest::get(sign_url).await?;
    println!("{:?}", resp);
    Ok(())
}

async fn post(url: &str) ->AnyResult<String> {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    let resp = client.post(url).headers(headers).send().await?.bytes().await?;
    let respStr = String::from_utf8_lossy(&resp);
    Ok(respStr.to_string())
}

