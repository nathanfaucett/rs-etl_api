use actix_web::{get, web::ServiceConfig, HttpResponse, Responder};

use crate::model;

#[utoipa::path(
    responses(
        (status = 200, description = "Get current user", body = UserResponse),
    ),
    security(
        ("Authorization" = [])
    )
)]
#[get("/users/current")]
pub async fn current(user: model::user::User) -> impl Responder {
    let user_response: model::user::UserResponse = user.into();
    HttpResponse::Ok().json(user_response)
}

pub fn configure() -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        config.service(current);
    }
}
