use serde::{Deserialize, Serialize};
use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};

use crate::settings::SETTINGS;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub iss: String,
    pub sub: String,
}

impl Claims {
    pub fn new(sub: String) -> Result<Self, SystemTimeError> {
        let now_in_seconds = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as usize;
        Ok(Claims {
            exp: now_in_seconds + SETTINGS.jwt.expiration_time_in_seconds,
            iat: now_in_seconds,
            iss: String::from("ETL"),
            sub,
        })
    }
}
