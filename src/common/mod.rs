//! 公共模块
//!
//! Auth: Mr.Wht
//! Date: 2023/02/18
//! Description: 公共模块
//! ```
use std::{sync::Arc, env};

use bollard::Docker;
use lazy_static::lazy_static;
use serde::Deserialize;
use config::{ConfigError, Config, File};

pub mod resp;
pub mod errors;

lazy_static!{
    pub static ref DL:Arc<Docker> = Arc::new(Docker::connect_with_socket_defaults().unwrap());
    pub static ref CONF:Arc<ConfigSetting> = Arc::new(init_config().unwrap());
}


#[derive(Deserialize)]
pub struct ConfigSetting {
    pub server: Server,
}

#[derive(Deserialize)]
pub struct Server {
    pub addr: String,
}


pub fn init_config() -> Result<ConfigSetting,ConfigError> {
    let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
    let conf = Config::builder()
        .add_source(File::with_name(&format!("config/{}",run_mode)))
        .build()?;
    conf.try_deserialize()
}