use actix_web::http::header::ContentType;
use diesel::{query_dsl::methods::SelectDsl, SelectableHelper};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use diesel_async::pooled_connection::deadpool::Pool;
use actix_web::{get, web, HttpResponse, Responder, error};
use serde::Serialize;

use crate::{models::recipe::Recipe, schema::recipes};

#[derive(Serialize)]
struct ResponseBodyVec<T> {
    pub result: T,
}

#[get("")]
pub async fn recipes_list(
    pool: web::Data<Pool<AsyncPgConnection>>,
) -> actix_web::Result<impl Responder> {
    let mut connection = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return Err(error::ErrorInternalServerError("Unable to get connection from pool")),
    };

    let db_result = recipes::table
        .select(Recipe::as_select())
        .load(&mut connection).await;
    let recipes_vec = match db_result {
        Ok(recipes) => recipes,
        Err(_) => return Err(error::ErrorInternalServerError("Cannot get recipes from the database")),
    };
    let response_body = ResponseBodyVec {
        result: recipes_vec,
    };
    let response_serialized = match serde_json::to_string(&response_body) {
        Ok(res) => res,
        Err(_) => return Err(error::ErrorInternalServerError("Cannot serialize recipes"))
    };

    return Ok(
        HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(response_serialized)
    );
}

pub fn recipes_config(cfg: &mut web::ServiceConfig) {
    cfg.service(recipes_list);
}
