use std::{error::Error, net::Ipv4Addr};

use actix_cors::Cors;
use actix_web::{middleware::Logger, App, HttpServer, web};
use utoipa::{
    openapi::security::{SecurityScheme, HttpBuilder, HttpAuthScheme},
    Modify, OpenApi,
};
use utoipa_swagger_ui::SwaggerUi;

use etl::{
    controller::util, 
    controller::user, 
    controller::auth, 
    model::user as model_user, 
    model::util as util_model, 
    model::auth as auth_model, 
    settings::SETTINGS, 
    middleware::auth::Authorization
};

#[actix_web::main]
async fn main() -> Result<(), impl Error> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    struct SecurityAddon;

    impl Modify for SecurityAddon {
        fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
            let components = openapi.components.as_mut().unwrap();
            components.add_security_scheme(
                "Authorization",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            )
        }
    }

    #[derive(OpenApi)]
    #[openapi(
        paths(
            util::health,
            util::version,
            user::current,
            auth::auth,
        ),
        components(
            schemas(
                util_model::VersionResponse, 
                util_model::HealthResponse, 
                util_model::ErrorResponse,
                model_user::UserResponse,
                auth_model::AuthRequest,
                auth_model::AuthResponse,
            )
        ),
        tags(
            (name = "util", description = "Utility endpoints"),
            (name = "user", description = "User endpoints")
        ),
        modifiers(&SecurityAddon)
    )]
    struct ApiDoc;

    let openapi = ApiDoc::openapi();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", openapi.clone()),
            )
            .wrap(
                Cors::default()
                    .allow_any_header()
                    .allow_any_method()
                    .allow_any_origin()
                    .expose_any_header()
                    .supports_credentials(),
            )
            .configure(util::configure())
            .configure(auth::configure())
            .service(
                web::scope("")
                    .wrap(Authorization)
                    .configure(user::configure())
            )
    })
    .bind((
        option_env!("HOST").unwrap_or("").parse::<Ipv4Addr>().ok().or(SETTINGS.server.host).unwrap_or(Ipv4Addr::UNSPECIFIED), 
        option_env!("PORT").unwrap_or("").parse::<u16>().unwrap_or(SETTINGS.server.port))
    )?
    .run()
    .await
}
