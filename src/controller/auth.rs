use actix_web::{
    post,
    web::{Json, ServiceConfig},
    HttpResponse, Responder,
};
use jsonwebtoken::{encode, EncodingKey, Header};

use crate::{jwt::Claims, model, settings::SETTINGS};

#[utoipa::path(
    request_body = AuthRequest,
    responses(
        (status = 201, description = "Get JWT", body = AuthResponse),
    )
)]
#[post("/auth")]
pub async fn auth(body: Json<model::auth::AuthRequest>) -> impl Responder {
    // TODO: validate username and password
    let claims = match Claims::new(body.username.clone()) {
        Ok(claims) => claims,
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };
    let jwt = match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SETTINGS.jwt.secret.as_bytes()),
    ) {
        Ok(jwt) => jwt,
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };
    HttpResponse::Ok().json(model::auth::AuthResponse { jwt })
}

pub fn configure() -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        config.service(auth);
    }
}
