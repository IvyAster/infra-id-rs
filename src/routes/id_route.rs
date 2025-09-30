use crate::common::common::ApiResult;
use crate::routes::RouteResult;
use crate::services::id_service::IdService;
use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/id", web::get().to(id_wrapper))
            .route("/id/struct/{id}", web::get().to(id_parse))
            .route("/ids/{size}", web::get().to(ids_wrapper)),
    );
    cfg.service(web::scope("/id").route("", web::get().to(id)));
    cfg.service(web::scope("/ids").route("/{size}", web::get().to(ids)));
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdStruct {
    timestamp: u64,
    worker_id: u64,
    sequence: u64,
}

async fn id(id_service: web::Data<IdService>) -> RouteResult<HttpResponse> {
    Ok(HttpResponse::Ok().body(id_service.id()))
}

async fn id_parse(
    id_service: web::Data<IdService>,
    path: web::Path<String>,
) -> RouteResult<ApiResult<IdStruct>> {
    let id = path.into_inner();
    let (timestamp, worker_id, sequence) = id_service.parse(&id);
    Ok(ApiResult::success(IdStruct {
        timestamp,
        worker_id,
        sequence,
    }))
}

async fn id_wrapper(id_service: web::Data<IdService>) -> RouteResult<ApiResult<String>> {
    Ok(ApiResult::success(id_service.id()))
}

async fn ids_wrapper(
    id_service: web::Data<IdService>,
    path: web::Path<u32>,
) -> RouteResult<ApiResult<Vec<String>>> {
    let numbers = path.into_inner();
    Ok(ApiResult::success(id_service.ids(numbers)))
}

async fn ids(id_service: web::Data<IdService>, path: web::Path<u32>) -> RouteResult<HttpResponse> {
    let numbers = path.into_inner();
    Ok(HttpResponse::Ok().json(id_service.ids(numbers)))
}
