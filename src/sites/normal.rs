use crate::conf::conf;
use anyhow::{Ok, Result as AnyResult};
use reqwest::header::{self};
use serde::{Deserialize, Serialize};

/// glodos 签到
#[derive(Debug, Deserialize, Serialize)]
struct SignOfGladosResp {
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
    let res_obj: Result<SignOfGladosResp, serde_json::Error> = serde_json::from_str(resp.as_str());

    // println!("{:#?}", res_obj);
    if res_obj.is_err() {
        eprintln!("sign err {:?}", res_obj.err());
    }
    Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct SignOfJueJinResp {
    pub err_no: i32,
    pub err_msg: String,
}

/// 签到发送 post 请求
pub(crate) async fn post<'a: 'static>(
    sign_url: &'a str,
    cookie: &'a str,
    req_body: &'a str,
) -> AnyResult<serde_json::Value> {
    // SignOfJueJinResp
    let client = reqwest::Client::new();
    let resp = client
        .post(sign_url)
        .header(header::COOKIE, cookie)
        .header(header::CONTENT_TYPE, "application/json; charset=utf-8")
        .body(req_body)
        .send()
        .await?
        .text()
        .await?;

    // let res_obj: Result<RESP, serde_json::Error> = serde_json::from_str(resp.as_str());
    // match res_obj {
    //     std::result::Result::Ok(v) => Ok(v),
    //     Err(err) => Err(anyhow::anyhow!(err)),
    // }
    let res_obj: serde_json::Value = serde_json::from_str(resp.as_str())?;
    println!("resp:{:#?}", res_obj.as_str());
    Ok(res_obj)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_post_juejin() {
        let sign_url = "https://api.juejin.cn/growth_api/v1/check_in";
        let cookie = r#"xxx"#;
        let req_body = "{}";
        let res = post(sign_url, cookie, req_body).await;
        assert_eq!(res.is_ok(), true);
    }
}
