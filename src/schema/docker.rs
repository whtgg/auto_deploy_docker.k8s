use std::ops::Deref;

use bollard::system::Version;
use serde::{Serialize, Deserialize};


/// docker版本信息
#[derive(Debug,Serialize,Deserialize)]
pub struct DockerVersionInfo {
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// The operating system that the daemon is running on (\"linux\" or \"windows\")
    #[serde(rename = "Os")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,

    #[serde(rename = "Arch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arch: Option<String>,

    #[serde(rename = "KernelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel_version: Option<String>,
}


/// 启动容器请求参数
#[derive(Deserialize)]
pub struct ReqRunContainer {
    pub image: String, // REPOSITORY:Tag
}

/// 请求关于容器相关
#[derive(Deserialize,Debug)]
pub struct ReqContainer {
    pub container_id: String,
}

/// 请求关于容器相关
#[derive(Deserialize,Debug)]
pub struct ReqExecContainer {
    pub command:Option<Vec<String>>,
    #[serde(flatten)]
    pub container_id: ReqContainer,
}

impl From<Version> for DockerVersionInfo {
    fn from(value: Version) -> Self {
        Self {
            version:value.version,
            os:value.os,
            arch: value.arch,
            kernel_version:value.kernel_version,
        }
    }
}

impl Deref for ReqContainer {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.container_id
    }
}