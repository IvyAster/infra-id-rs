use actix_web::{HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResult<T> {
    /// 业务状态码
    pub code: String,
    /// 响应消息
    pub message: String,
    /// 响应数据
    pub data: Option<T>,
}

impl<T> ApiResult<T> {
    pub fn error(message: &str) -> ApiResult<T> {
        ApiResult {
            code: "200".to_string(),
            message: message.to_string(),
            data: None,
        }
    }

    pub fn success(data: T) -> ApiResult<T> {
        ApiResult {
            code: "200".to_string(),
            message: "success".to_string(),
            data: Some(data),
        }
    }
}

impl<T: Serialize> Responder for ApiResult<T> {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pagination<T> {
    pub page: u32,
    pub size: u32,
    pub pages: u64,
    pub total: u64,
    pub data: Option<Vec<T>>,
}
