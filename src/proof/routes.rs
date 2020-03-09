use crate::api_error::ApiError;
use crate::proof::Proof;
use actix_web::{delete, get, web, HttpResponse};
use serde_json::json;

#[get("/proofs")]
async fn find_all() -> Result<HttpResponse, ApiError> {
    let proofs = Proof::find_all()?;
    Ok(HttpResponse::Ok().json(proofs))
}

#[get("/proofs/{id}")]
async fn find(id: web::Path<String>) -> Result<HttpResponse, ApiError> {
    let proof = Proof::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(proof))
}

#[delete("/proofs/{id}")]
async fn delete(id: web::Path<String>) -> Result<HttpResponse, ApiError> {
    let num_deleted = Proof::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": num_deleted })))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(find);
    cfg.service(delete);
}
