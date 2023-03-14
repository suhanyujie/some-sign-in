use anyhow::{Result as AnyResult, Ok};

/// 启动一个定时任务服务
/// 每天执行签到请求 todo


#[tokio::main]
async fn main() {
    req_sign_in();
}

async fn req_sign_in() ->AnyResult<()> {
    let sign_url = "https://glados.rocks/console";
    let resp = reqwest::get(sign_url).await?;
    println!("{:?}", resp);
    Ok(())
}

