use crate::common::common::ApiResult;
use actix_web::body::EitherBody;
use actix_web::{
    HttpResponse, ResponseError, body::BoxBody, dev::ServiceResponse, http::StatusCode,
    middleware::ErrorHandlerResponse,
};
use thiserror::Error;

pub fn error_handlers() -> actix_web::middleware::ErrorHandlers<BoxBody> {
    actix_web::middleware::ErrorHandlers::new()
        .handler(StatusCode::NOT_FOUND, error_handler)
        .handler(StatusCode::INTERNAL_SERVER_ERROR, error_handler)
        .handler(StatusCode::BAD_REQUEST, error_handler)
        .handler(StatusCode::UNAUTHORIZED, error_handler)
        .handler(StatusCode::FORBIDDEN, error_handler)
}

// 统一的JSON错误处理器
fn error_handler<B>(
    res: ServiceResponse<B>,
) -> Result<ErrorHandlerResponse<BoxBody>, actix_web::Error> {
    let status = res.status();
    let api_result = ApiResult::<()>::error(status.canonical_reason().unwrap_or("Unknown error"));

    let json_response = serde_json::to_string(&api_result)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(ErrorHandlerResponse::Response(
        res.map_body(|_head, _body| BoxBody::new(json_response))
            .map_body(|_header, body| EitherBody::left(body)),
    ))
}

// 自定义应用错误枚举
#[derive(Debug, Error)]
pub enum AppError {
    // 包装 anyhow::Error 的变体
    // 使用 thiserror 的 #[from] 属性自动实现 From<anyhow::Error>
    #[error("An unexpected error occurred: {0}")]
    AnyhowError(#[from] anyhow::Error),
}

// 为 AppError 实现 ResponseError，这是与 Actix-web 集成的关键
impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        // 根据错误类型，返回不同的 HTTP 状态码和消息
        let (status_code, message) = match self {
            AppError::AnyhowError(e) => {
                // 在生产环境中，你可能不想暴露内部错误细节
                // 这里我们简化处理，直接返回错误信息
                // 在生产环境，建议使用 "Internal Server Error" 并记录日志
                (StatusCode::OK, e.to_string())
            }
        };

        HttpResponse::build(status_code)
            .content_type("application/json")
            .json(ApiResult::<()>::error(&message))
    }
}
