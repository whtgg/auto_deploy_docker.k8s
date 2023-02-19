//! 路由处理
//!
//! Auth: Mr.Wht
//! Date: 2023/02/18
//! Description: 路由处理
//! ```

use std::{convert::Infallible, borrow::Cow};

use axum::{Router,routing::{get,post},body::{BoxBody, HttpBody}, response::{IntoResponse,Response}, http::{StatusCode, Uri}};
use tower::util::AndThenLayer;

use crate::common::resp::api_resp_fail;
use crate::api::docker::*;

pub fn api_router() -> Router{
    Router::new()
        .route("/", get(version))
        .route("/images", get(images))
        .route("/containers", get(containers))
        .route("/build", post(build))
        .route("/start",post(start))
        .route("/exec",post(exec))
        .route("/state",get(state))
        .layer(AndThenLayer::new(map_response))
        .fallback(fallback)
}

async fn fallback(_uri: Uri) -> impl IntoResponse {
    api_resp_fail::<String>(StatusCode::NOT_FOUND.as_u16() as u32, "请求接口不存在").into_response()
}


/// 响应处理
async fn map_response(response: Response<BoxBody>) -> std::result::Result<Response<BoxBody>, Infallible>
{
    const DEFAULT_ERROR: &str = "Internal Server Error";
    let status_code = response.status();
    if status_code.is_success() {
        Ok(response)
    } else {
        let (_,mut body)  = response.into_parts();
        let body = body.data().await;
        let message: Cow<'_, str> = match &body {
            Some(body) => match &body {
                Ok(body) => std::str::from_utf8(body)
                    .ok()
                    .unwrap_or(DEFAULT_ERROR)
                    .into(),
                Err(e) => e.to_string().into(),
            },
            None => status_code
                .canonical_reason()
                .unwrap_or(DEFAULT_ERROR)
                .into(),
        };
        let code = status_code.as_u16().into();
        let body = api_resp_fail::<String>(code, &message).into_response();
        Ok(body)
    }
}

