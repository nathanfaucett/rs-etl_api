use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub enum ErrorResponse {
    NotFound(String),
    Conflict(String),
    Unauthorized(String),
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct HealthResponse {
    pub ok: bool,
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct VersionResponse<'a> {
    pub version: &'a str,
}
