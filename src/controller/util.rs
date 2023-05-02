use crate::model;
use actix_web::{get, web::ServiceConfig, HttpResponse, Responder};

#[utoipa::path(
    responses(
        (status = 200, description = "Health check response", body = [HealthResponse]),
    )
)]
#[get("/health")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok().json(model::util::HealthResponse { ok: true })
}

#[utoipa::path(
    responses(
        (status = 200, description = "Version response", body = [VersionResponse]),
    )
)]
#[get("/version")]
pub async fn version() -> impl Responder {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    HttpResponse::Ok().json(model::util::VersionResponse { version: VERSION })
}

pub fn configure() -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        config.service(health).service(version);
    }
}
