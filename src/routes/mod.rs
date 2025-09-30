use actix_web::web;

pub mod id_route;

pub fn config(cfg: &mut web::ServiceConfig) {
    id_route::config(cfg);
}

use crate::handler::error_handler::AppError;

pub type RouteResult<T, E = AppError> = Result<T, E>;
