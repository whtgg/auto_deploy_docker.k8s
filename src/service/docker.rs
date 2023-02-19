
use std::io::Write;

use axum::body::Bytes;
use bollard::{system::Version, image::{ListImagesOptions, BuildImageOptions}, service::{ImageSummary, ContainerSummary}, container::{ListContainersOptions,StatsOptions, Stats, Config}, exec::{CreateExecOptions, StartExecResults}};
use futures_util::{TryStreamExt, StreamExt};

use crate::common::{errors::{Result, MessageError}, DL};

/// 当前服务器上docker信息
pub async fn get_version() -> Result<Version>{
    let version = DL.version().await?;
    Ok(version)
}

/// docker镜像列表
pub async fn list_images() -> Result<Vec<ImageSummary>> {
    Ok(DL.list_images(Some(ListImagesOptions::<String>{
        all: true,
        ..Default::default()
    })).await?)
}

/// docker 容器列表
pub async fn list_container() -> Result<Vec<ContainerSummary>> {
    Ok(DL.list_containers(Some(ListContainersOptions::<String>{
        all: true,
        ..Default::default()
    })).await?)
}

/// 用dockerfile 构建镜像
pub async fn build_image(body:Bytes) -> Result<()> {
    let mut header = tar::Header::new_gnu();
    let body_string = std::str::from_utf8(&body).unwrap();
    header.set_path("Dockerfile").unwrap();
    header.set_size(body.len() as u64);
    header.set_mode(0o755);
    header.set_cksum();
    let mut tar = tar::Builder::new(Vec::new());
    tar.append(&header, body_string.as_bytes()).unwrap();

    let uncompressed = tar.into_inner().unwrap();
    let mut c = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    c.write_all(&uncompressed).unwrap();
    let compressed = c.finish().unwrap();
    tracing::info!("dockerfile-content --- {:?}",compressed);
    
    let build_image_options = BuildImageOptions {
        dockerfile: "Dockerfile",
        rm: true,
        ..Default::default()
    };
    let mut image_build_stream =
    DL.build_image(build_image_options, None, Some(compressed.into()));
    while let Some(msg) = image_build_stream.next().await {
        println!("Message: {:?}", msg);
    }
    Ok(())
}

/// 用dockerfile 启动容器
pub async fn build_container(image:&str) -> Result<()> {
    let id = DL
        .create_container::<&str, &str>(None, Config{
            image: Some(image),
            tty: Some(true),
            ..Default::default()
        })
        .await?
        .id;
    DL.start_container::<String>(&id, None).await?;
    Ok(())
}

/// 进入容器
pub async fn exec_container(container_id:&str,command:Option<Vec<String>>) -> Result<()> {
    let exec = DL
        .create_exec(
            container_id,
            CreateExecOptions {
                attach_stdout: Some(true),
                attach_stderr: Some(true),
                cmd: command,
                ..Default::default()
            },
        )
        .await?
        .id;

    if let StartExecResults::Attached { mut output, .. } = DL.start_exec(&exec, None).await? {
        while let Some(Ok(msg)) = output.next().await {
            println!("{}", msg);
        }
    } else {
       return Err(MessageError::new("容器执行错误"));
    }
    Ok(())
}

/// docker 容器运行状态
pub async fn state_container(container_id:&str) -> Result<Vec<Stats>> {
    let stats = DL.stats(container_id, Some(StatsOptions {
        stream: false,
        one_shot: true,
     })).try_collect::<Vec<_>>().await?;
     for stat in &stats {
        println!("{} - mem total: {:?} | mem usage: {:?}",
            stat.name,
            stat.memory_stats.max_usage,
            stat.memory_stats.usage);
    }
    Ok(stats)
}

/// docker 关闭运行中的container
pub async fn stop_container(container_id:&str) -> Result<()> {
    DL.stop_container(container_id, Default::default()).await?;
    Ok(())
}