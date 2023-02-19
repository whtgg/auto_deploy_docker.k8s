//! 路由处理
//!
//! Auth: Mr.Wht
//! Date: 2023/02/18
//! Description: 路由处理
//! ```

use std::{convert::Infallible, borrow::Cow};

use axum::{Router,routing::{get,post},body::{BoxBody, HttpBody, Body}, response::{IntoResponse,Response}, middleware::{from_fn, Next}, http::{Request, StatusCode, Uri}};
use tower::util::AndThenLayer;
use serde::Deserialize;

use crate::common::resp::api_resp_fail;
use crate::common::errors::Result;
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
        .layer(from_fn(request_auth_token))
        .fallback(fallback)
}


#[derive(Deserialize,Debug)]
pub struct Hello {
    pub username: String,
}

async fn fallback(_uri: Uri) -> impl IntoResponse {
    api_resp_fail::<String>(StatusCode::NOT_FOUND.as_u16() as u32, "请求接口不存在").into_response()
}

async fn request_auth_token(req:Request<Body>,next:Next<Body>) -> Result<impl IntoResponse, Response> 
{
    let headers = req.headers();
    let token = headers.get("token");
    if token.is_some() &&  token.unwrap().to_str().unwrap().eq("ok") {
        return Ok(api_resp_fail::<String>(500, "").into_response());
    }
    Ok(next.run(req).await)
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

