use anyhow::{Ok, Result as AnyResult};
use core::time;
use delay_timer::prelude::*;
use rand::Rng;
#[macro_use]
extern crate lazy_static;

mod conf;
mod sites;

/// 启动一个定时任务服务
/// 每天执行签到请求

#[tokio::main]
async fn main() {
    println!("定时任务已启动，到达特定时间点后会进行签到，请不要关闭窗口...");
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
        let sleep_sec = rand::thread_rng().gen_range(0..15);
        std::thread::sleep(time::Duration::from_secs(sleep_sec));
        match sites::normal::req_sign_in().await {
            std::result::Result::Ok(_) => {
                println!("glados 签到完成 ok...");
            }
            std::result::Result::Err(err) => {
                eprintln!("glados 签到失败 error: {:#?}", err);
            }
        }
        // 其他签到 todo
        let sign_list = &conf::conf::GLOBAL_CONFIG.user.sign_list;
        for item in sign_list {
            if item.site_name.is_empty() {
                continue;
            }
            let res = sites::normal::post(&item.url, &item.cookie, &item.req_body).await;
            println!("------------- {} 签到：-----------------", item.site_name);
            match res {
                std::result::Result::Ok(v) => {
                    println!("{}", &v.err_msg)
                }
                Err(err) => {
                    eprintln!("{}", err)
                }
            }
            println!("------------------------------")
        }
        println!("本次签到任务完成，等待下次任务时间...")
    };
    task_builder
        .set_frequency_repeated_by_cron_str(expr)
        .set_task_id(1)
        .set_maximum_running_time(20)
        .spawn_async_routine(body)
}
