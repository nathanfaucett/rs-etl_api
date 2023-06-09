use actix_web::{dev::Payload, FromRequest, HttpMessage, HttpRequest};
use chrono::{DateTime, Utc};
use futures::{future::err, future::ok};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct UserResponse {
    pub id: usize,
    pub username: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: usize,
    pub username: String,
    pub encrypted_password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Into<UserResponse> for User {
    fn into(self) -> UserResponse {
        UserResponse {
            id: self.id,
            username: self.username,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

impl FromRequest for User {
    type Error = actix_web::Error;
    type Future = futures::future::Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        match req.extensions().get::<User>() {
            Some(user) => ok(user.clone()),
            None => err(actix_web::error::ErrorUnauthorized("invalid_user")),
        }
    }
}
