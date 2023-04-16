use anyhow::{Ok, Result as AnyResult};
use reqwest::header::{self, HeaderMap};
use serde::{Deserialize, Serialize};
#[macro_use]
use delay_timer::prelude::*;
#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref GLOBAL_CONFIG: Config = {
        match read_config() {
            std::result::Result::Ok(config_obj) => config_obj,
            _ => {
                panic!("read config err")
            }
        }
    };
}

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

async fn req_sign_in() -> AnyResult<()> {
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

async fn post(url: &str) -> AnyResult<String> {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    let resp = client
        .post(url)
        .headers(headers)
        .send()
        .await?
        .bytes()
        .await?;
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
    cron_expr: String,
}

fn read_config() -> AnyResult<Config> {
    let data = std::fs::read_to_string("./env.local.toml")?;
    let obj: Config = toml::from_str(&data)?;
    Ok(obj)
}

#[tokio::main]
async fn main() {
    exec_interval();
}

fn exec_interval() -> AnyResult<()> {
    let deplay_timer = DelayTimerBuilder::default().build();
    let task_instance_chain = deplay_timer.insert_task(build_task()?)?;
    let task_instance = task_instance_chain.next_with_wait()?;
    // task_instance.cancel_with_wait_timeout()?;
    std::thread::park();
    Ok(())
}

fn build_task() -> AnyResult<Task, TaskError> {
    let expr = &GLOBAL_CONFIG.sys.cron_expr;
    let mut task_builder = TaskBuilder::default();
    let body = || async {
        println!("task exec start...");
        match req_sign_in().await {
            std::result::Result::Ok(()) => {
                println!("ok...");
            }
            std::result::Result::Err(err) => {
                eprintln!("error: {:#?}", err);
            }
        };
        println!("task exec end...");
    };
    task_builder
        .set_frequency_repeated_by_cron_str(expr)
        .set_task_id(1)
        .set_maximum_running_time(20)
        .spawn_async_routine(body)
}
