mod recipes_service;
mod recipes_web;

use actix_web::{middleware::Logger, web, App, HttpServer};
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use dotenvy::dotenv;
use std::{env, io};
use utoipa_actix_web::{scope, AppExt};
use utoipa_swagger_ui::SwaggerUi;

use recipes_web::controllers::{categories::categories_config, recipes::recipes_config};

const API_PREFIX: &str = "/api/v1";

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool_manager =
        AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);
    let pool = Pool::builder(pool_manager)
        .build()
        .expect("Unable to build a connection pool");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .into_utoipa_app()
            .app_data(web::Data::new(pool.clone()))
            .service(
                scope(API_PREFIX)
                    .service(scope("/recipes").configure(recipes_config))
                    .service(scope("/categories").configure(categories_config)),
            )
            .openapi_service(|api| {
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api/openapi.json", api)
            })
            .into_app()
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
