//! 错误定义模块
//!
//! Auth: Mr.Wht
//! Date: 2023/02/18
//! Description: 错误定义模块
//! ```

use std::{convert::Infallible, fmt::Debug};

use axum::{response::IntoResponse, extract::{rejection::{JsonDataError, JsonRejection, JsonSyntaxError}, multipart::MultipartError}, http::{StatusCode, uri::InvalidUri}};
use serde::{Serialize, Deserialize};
use bollard::errors::Error as DockerError;

use super::resp::api_resp_fail;

// 定义alias
pub type Result<T,E=MessageError> = std::result::Result<T,E>;
// 定义服务器内部错误
const CODE_SERVER_ERROR:i32 = 500;

#[derive(Debug,Serialize,Deserialize)]
pub enum MessageError{
     HttpRequestError(Option<i32>,String),
     Common(String),
} 

impl MessageError {
    pub fn new(msg:&str) -> Self {
        MessageError::Common(msg.to_string())
    }
}

impl IntoResponse for MessageError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            MessageError::HttpRequestError(code, msg) => {
                (code.unwrap_or(CODE_SERVER_ERROR), msg)
            },
            MessageError::Common(msg) => {
                (CODE_SERVER_ERROR, msg)
            },
        };
        api_resp_fail::<String>(status as u32,&error_message).into_response()
    }
}

impl From<JsonDataError> for MessageError {
    fn from(e: JsonDataError) -> Self {
        Self::HttpRequestError(None, e.to_string())
    }
}


impl From<JsonSyntaxError> for MessageError {
    fn from(e: JsonSyntaxError) -> Self {
        Self::HttpRequestError(None, e.to_string())
    }
}

impl From<Infallible> for MessageError {
    fn from(_: Infallible) -> Self {
        Self::HttpRequestError(None, "Infallible".into())
    }
}

impl From<MultipartError> for MessageError {
    fn from(e: MultipartError) -> Self {
        Self::HttpRequestError(None, e.to_string())
    }
}

impl From<std::option::Option<Infallible>> for MessageError {
    fn from(e: std::option::Option<Infallible>) -> Self {
        if let Some(infallible) = e {
            Self::HttpRequestError(None, infallible.to_string())
        } else {
            Self::HttpRequestError(None, "Infallible".to_string())
        }
    }
}

impl From<InvalidUri> for MessageError {
    fn from(e: InvalidUri) -> Self {
        Self::HttpRequestError(None, e.to_string())
    }
}

impl From<serde_json::Error> for MessageError {
    fn from(e: serde_json::Error) -> Self {
        Self::HttpRequestError(None, e.to_string())
    }
}

impl From<axum::http::Error> for MessageError {
   fn from(e: axum::http::Error) -> Self {
    Self::HttpRequestError(None, e.to_string())
   }
}

impl From<JsonRejection> for MessageError {
    fn from(rejection: JsonRejection) -> Self {
        let code = match rejection {
            JsonRejection::JsonDataError(_) => StatusCode::UNPROCESSABLE_ENTITY,
            JsonRejection::JsonSyntaxError(_) => StatusCode::BAD_REQUEST,
            JsonRejection::MissingJsonContentType(_) => StatusCode::UNSUPPORTED_MEDIA_TYPE,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        Self::HttpRequestError(Some(code.as_u16() as i32), "Infallible".into())
    }
}

impl From<DockerError> for MessageError {
    fn from(e: DockerError) -> Self {
        Self::HttpRequestError(None, e.to_string())
    }
}


