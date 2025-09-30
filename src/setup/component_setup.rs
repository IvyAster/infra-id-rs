use actix_web::web;
use crate::services::id_service::IdService;
use crate::services::IService;

pub fn setup(worker_id: u64) -> Vec<web::Data<impl IService>>{
    let mut vec = Vec::new();
    let id_service = web::Data::new(IdService::new(worker_id));
    vec.push(id_service.clone());
    vec
}
