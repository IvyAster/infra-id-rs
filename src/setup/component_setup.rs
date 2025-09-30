use crate::services::IService;
use crate::services::id_service::IdService;
use actix_web::web;

pub fn setup(worker_id: u64) -> Vec<web::Data<impl IService>> {
    let mut vec = Vec::new();
    let id_service = web::Data::new(IdService::new(worker_id));
    vec.push(id_service.clone());
    vec
}
