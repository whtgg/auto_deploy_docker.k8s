//! 请求和响应处理
//!
//! Auth: Mr.Wht
//! Date: 2023/02/18
//! Description: 请求和响应处理
//! ```

use std::ops::Deref;

use axum::{response::IntoResponse, http::{StatusCode, Request}, Json, body::{HttpBody, Bytes},extract::FromRequest, BoxError,async_trait};
use serde::{Serialize, Deserialize, de::DeserializeOwned};
use super::errors::Result;

#[derive(Serialize, Deserialize,Clone)]
#[serde(rename_all = "camelCase")]
pub struct BaseResponse<T> {
   pub  code: u32,
   pub  message: String,
   pub  data: Option<T>,
}

/// 接口成功
pub fn api_resp_sucess<'de,T:Serialize + Deserialize<'de>>(data:T) -> Result<BaseResponse<T>> 
{
    Ok(BaseResponse{
        code:200,
        message:"操作成功".to_string(),
        data:Some(data),
    })
}

/// 接口失败
pub fn api_resp_fail<'de,T:Serialize + Deserialize<'de>>(code:u32,message:&str) -> Result<BaseResponse<T>> 
{
    Ok(BaseResponse{
        code,
        message:message.to_string(),
        data:None,
    })
}

impl<'de,T:Serialize + Deserialize<'de>> IntoResponse for BaseResponse<T> {
    fn into_response(self) -> axum::response::Response {
        let status_code = StatusCode::OK;
        let response = Json(self);
        let mut response = response.into_response();
        *response.status_mut() = status_code;
        response
    }
}

#[derive(Debug,Serialize,Deserialize)]
pub struct ApiReq<T> {
    pub page_no: Option<usize>,
    pub page_size: Option<usize>,
    #[serde(flatten)]
    pub params: T,
}

impl<T> ApiReq<T> {
    pub fn page(&self) -> usize{
        self.page_no.unwrap_or(1)
    }
    pub fn size(&self) -> usize{
        self.page_size.unwrap_or(20)
    }
}

impl<T> Deref for ApiReq<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.params
    }
}

/// 自动解析请求参数
#[async_trait]
impl<S, B, T> FromRequest<S, B> for ApiReq<T>
where

    T: DeserializeOwned + Send,
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
    S: Send + Sync,
{
    type Rejection = (StatusCode,String);

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        tracing::info!("****** {:?}",req.headers());
        let bytes =  Bytes::from_request(req, state).await.map_err(|e| (StatusCode::BAD_REQUEST,e.to_string()))?;
        let str = String::from_utf8(bytes.to_vec()).map_err(|e| (StatusCode::BAD_REQUEST,e.to_string()))?;
        let data:T = serde_json::from_str(&str).map_err(|e| (StatusCode::BAD_REQUEST,e.to_string()))?;
        Ok(ApiReq { page_no:Some(1), page_size: Some(20), params:data})
    }
}
