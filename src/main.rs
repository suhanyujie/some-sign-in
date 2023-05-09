use anyhow::{Ok, Result as AnyResult};
use chrono::{DateTime, FixedOffset, Local, Utc};
use core::time;
use delay_timer::prelude::*;
use rand::Rng;

use crate::conf::conf::OneSiteSign;
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
        let local_time = Local::now();
        let utc_time = DateTime::<Utc>::from_utc(local_time.naive_utc(), Utc);
        let china_timezone = FixedOffset::east_opt(8 * 3600).unwrap();
        println!(
            "------------- 新一轮签到 {} -------------",
            utc_time.with_timezone(&china_timezone)
        );

        let sleep_sec = rand::thread_rng().gen_range(0..15);
        std::thread::sleep(time::Duration::from_secs(sleep_sec));

        let sign_list = &conf::conf::GLOBAL_CONFIG.user.sign_list;
        let mut suc_num = 0;

        if sign_list.is_some() {
            let sign_list_ref = sign_list.as_ref().unwrap();
            for item in sign_list_ref {
                if item.site_name.is_empty() {
                    continue;
                }
                let res = sites::normal::post(&item.url, &item.cookie, &item.req_body).await;
                println!("------------- {} 签到：-----------------", item.site_name);
                match res {
                    std::result::Result::Ok(v) => {
                        println!("成功！{}", &v)
                    }
                    Err(err) => {
                        eprintln!("失败！{}", err)
                    }
                }
                println!("------------------------------");
                suc_num += 1;
            }
        }
        println!("本次签到任务完成 {} 个，等待下次任务时间...", suc_num);
    };
    task_builder
        .set_frequency_repeated_by_cron_str(expr)
        .set_task_id(1)
        .set_maximum_running_time(20)
        .spawn_async_routine(body)
}
