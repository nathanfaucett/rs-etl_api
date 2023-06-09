use std::future::{self, Ready};

use actix_web::{
    body::EitherBody,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    HttpMessage, HttpResponse,
};
use futures::future::LocalBoxFuture;
use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::{
    jwt::Claims,
    model::{self, util::ErrorResponse},
    settings::SETTINGS,
};

pub struct Authorization;

impl<S, B> Transform<S, ServiceRequest> for Authorization
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = actix_web::Error;
    type InitError = ();
    type Transform = AuthorizationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        future::ready(Ok(AuthorizationMiddleware { service }))
    }
}

pub struct AuthorizationMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthorizationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &self,
        ctx: &mut core::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        match req.headers().get("Authorization") {
            None => {
                let res =
                    req.into_response(HttpResponse::Unauthorized().json(
                        ErrorResponse::Unauthorized(String::from("missing_authorization")),
                    ))
                    .map_into_right_body();
                return Box::pin(async move { Ok(res) });
            }
            Some(jwt_header) => {
                let jwt = match jwt_header.to_str() {
                    Ok(jwt) => &jwt["Bearer ".len()..jwt.len()],
                    Err(err) => {
                        log::error!("Error: {}", err);
                        let res = req
                            .into_response(HttpResponse::Unauthorized().json(
                                ErrorResponse::Unauthorized(String::from("invalid_authorization")),
                            ))
                            .map_into_right_body();
                        return Box::pin(async move { Ok(res) });
                    }
                };
                let token = match decode::<Claims>(
                    jwt,
                    &DecodingKey::from_secret(SETTINGS.jwt.secret.as_bytes()),
                    &Validation::default(),
                ) {
                    Ok(token) => token,
                    Err(err) => {
                        log::error!("Error: {}", err);
                        let res = req
                            .into_response(HttpResponse::Unauthorized().json(
                                ErrorResponse::Unauthorized(String::from("invalid_authorization")),
                            ))
                            .map_into_right_body();
                        return Box::pin(async move { Ok(res) });
                    }
                };
                // TODO: fetch real user
                let user = model::user::User {
                    id: 0,
                    username: token.claims.sub,
                    encrypted_password: String::from(""),
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                };
                req.extensions_mut().insert(user);
            }
        }

        let future = self.service.call(req);

        Box::pin(async move {
            let res = future.await?.map_into_left_body();
            Ok(res)
        })
    }
}
