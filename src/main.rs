use core::time;

use anyhow::{Ok, Result as AnyResult};
use rand::Rng;
use reqwest::header::{self, HeaderMap};
use serde::{Deserialize, Serialize};
#[macro_use]
use delay_timer::prelude::*;
#[macro_use]
extern crate lazy_static;

mod sites;
mod conf;

/// 启动一个定时任务服务
/// 每天执行签到请求

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

#[tokio::main]
async fn main() {
    println!("start exec cycle...");
    exec_interval();
}

fn exec_interval() -> AnyResult<()> {
    let deplay_timer = DelayTimerBuilder::default().build();
    let _ = deplay_timer.insert_task(build_task()?)?;
    std::thread::park();
    Ok(())
}

fn build_task() -> AnyResult<Task, TaskError> {
    let expr = &conf::conf::GLOBAL_CONFIG.sys.cron_expr;
    let mut task_builder = TaskBuilder::default();
    let body = || async {
        let sleep_sec = rand::thread_rng().gen_range(0..50);
        std::thread::sleep(time::Duration::from_secs(sleep_sec));
        println!("开始签到...");
        match sites::normal::req_sign_in().await {
            std::result::Result::Ok(()) => {
                println!("ok...");
            }
            std::result::Result::Err(err) => {
                eprintln!("error: {:#?}", err);
            }
        };
        println!("签到完成...");
    };
    task_builder
        .set_frequency_repeated_by_cron_str(expr)
        .set_task_id(1)
        .set_maximum_running_time(20)
        .spawn_async_routine(body)
}
