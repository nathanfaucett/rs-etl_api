use std::{error::Error, net::Ipv4Addr};

use actix_cors::Cors;
use actix_web::{middleware::Logger, App, HttpServer};
use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_swagger_ui::SwaggerUi;

use etl_api::{controller::util, model::util as util_model};

#[actix_web::main]
async fn main() -> Result<(), impl Error> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    struct SecurityAddon;

    impl Modify for SecurityAddon {
        fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
            let components = openapi.components.as_mut().unwrap();
            components.add_security_scheme(
                "api_jwt_token",
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
        ),
        components(
            schemas(
                util_model::VersionResponse, 
                util_model::HealthResponse, 
                util_model::ErrorResponse
            )
        ),
        tags(
            (name = "util", description = "Utility endpoints")
        ),
        modifiers(&SecurityAddon)
    )]
    struct ApiDoc;

    let openapi = ApiDoc::openapi();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .send_wildcard()
                    .block_on_origin_mismatch(false),
            )
            .configure(util::configure())
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
    })
    .bind((
        Ipv4Addr::UNSPECIFIED, 
        option_env!("PORT").unwrap_or("").parse::<u16>().unwrap_or(8080))
    )?
    .run()
    .await
}
