use crate::api_error::ApiError;
use crate::fence::{Fence, FenceMessage};
use crate::proof::{Proof};
use futures::{StreamExt};
use std::process::Command;
use std::fs::File;
use std::io::prelude::*;
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::{json, Value};
use blake3::hash;
use uuid::Uuid;

#[get("/fences")]
async fn find_all() -> Result<HttpResponse, ApiError> {
    let fences = Fence::find_all()?;
    Ok(HttpResponse::Ok().json(fences))
}

#[get("/fences/{id}")]
async fn find(id: web::Path<Uuid>) -> Result<HttpResponse, ApiError> {
    let user = Fence::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}

#[post("/fences")]
async fn create(fence: web::Json<FenceMessage>) -> Result<HttpResponse, ApiError> {
    let fence = Fence::create(fence.into_inner())?;
    Ok(HttpResponse::Ok().json(fence))
}

#[put("/fences/{id}")]
async fn update(id: web::Path<Uuid>, fence: web::Json<Fence>) -> Result<HttpResponse, ApiError> {

    let user = Fence::update(id.into_inner(), fence.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}

#[delete("/fences/{id}")]
async fn delete(id: web::Path<Uuid>) -> Result<HttpResponse, ApiError> {
    let num_deleted = Fence::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": num_deleted })))
}

#[post("/seek/{id}")]
async fn seek(id: web::Path<Uuid>, mut proof: web::Payload) -> Result<HttpResponse, ApiError> {
    let id = id.into_inner();
    let mut fence = Fence::find(id.clone())?;

    let mut bytes = web::BytesMut::new();
    while let Some(item) = proof.next().await {
        bytes.extend_from_slice(&item.map_err(|_| ApiError::new(404, "hello_error".to_string()))?);
    }

    let mut proof_hash = format!("{:?}", hash(&bytes).to_hex());

    let mut file = File::create("compute.json").expect("file could not be created");
    file.write_all(&bytes).expect("could not write to file");

    let output = Command::new("/home/yasushi/go/bin/hide_and_seek")
        .args(&["seek","-p", "compute.json"])
        .output()
        .expect("failed to run command");

    let result = String::from_utf8(output.stdout).expect("could not convert proof");

    let result = result.trim() == "true";

    let proof = Proof::new(proof_hash, id, result);
    let proof = Proof::create(proof)?;

    if result {
        fence.tcount += 1;
    } else {
        fence.fcount += 1;
    }

    // let fm = FenceMessage::from(fence);
    Fence::update(id, fence);

    std::fs::remove_file("compute.json").expect("could not remove file");

    Ok(HttpResponse::Ok().json(json!(proof)))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(find);
    cfg.service(create);
    cfg.service(update);
    cfg.service(delete);
    cfg.service(seek);
}
