use serde::{Deserialize, Serialize};
use anyhow::{Ok, Result as AnyResult};

lazy_static! {
    pub static ref GLOBAL_CONFIG: Config = {
        match read_config() {
            std::result::Result::Ok(config_obj) => config_obj,
            _ => {
                panic!("read config err")
            }
        }
    };
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub user: ConfigUser,
    pub sys: ConfigSys,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigUser {
    pub cookie: String,
    pub sign_req_body: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigSys {
    pub sign_url: String,
    pub cron_expr: String,
}

pub fn read_config() -> AnyResult<Config> {
    let data = std::fs::read_to_string("./env.local.toml")?;
    let obj: Config = toml::from_str(&data)?;
    Ok(obj)
}