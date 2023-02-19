//! docker相关接口
//!
//! Auth: Mr.Wht
//! Date: 2023/02/18
//! Description: docker相关接口
//! ```

use axum::extract::Multipart;
use bollard::{service::{ImageSummary, ContainerSummary}, container::Stats};

use crate::{common::{errors:: Result, resp::{BaseResponse, api_resp_sucess, ApiReq}}, service::docker::{get_version, list_images, state_container, build_image, build_container, list_container, exec_container}, schema::docker::{DockerVersionInfo, ReqRunContainer, ReqContainer, ReqExecContainer}};

/// docker 版本 相当于 docker version
pub async fn version() -> Result<BaseResponse<DockerVersionInfo>> 
{
    api_resp_sucess(get_version().await?.into())
}

/// docker 镜像 相当于 docker image
pub async fn images() -> Result<BaseResponse<Vec<ImageSummary>>> 
{
    api_resp_sucess(list_images().await?)
}

/// 构建镜像 相当于 docker build -t name .( -f path)
pub async fn build(mut multipart: Multipart) -> Result<BaseResponse<()>> {
    while let Some(field) = multipart.next_field().await? {
        build_image(field.bytes().await?).await?;
    }
    api_resp_sucess(())
}

/// docker 容器列表 相当于 docker ps -a 
pub async fn containers() -> Result<BaseResponse<Vec<ContainerSummary>>> 
{
    api_resp_sucess(list_container().await?)
}

/// 启动容器 相当于 docker run --name xxx -dit image
pub async fn start(image:ApiReq<ReqRunContainer>) -> Result<BaseResponse<()>> {
    build_container(image.image.as_str()).await?;
    api_resp_sucess(())
}

/// 启动容器 相当于 docker exec -it container_id /bin/bash
pub async fn exec(ex:ApiReq<ReqExecContainer>) -> Result<BaseResponse<()>> {
    api_resp_sucess(exec_container((*ex.container_id).as_str(),ex.command.clone()).await?)
}

/// 容器日志 相当于 docker stats container_id
pub async fn state(container_id:ApiReq<ReqContainer>) -> Result<BaseResponse<Vec<Stats>>> 
{
    api_resp_sucess(state_container(container_id.container_id.as_str()).await?)
}