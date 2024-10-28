use actix_web::{
    delete, error, get,
    http::{header::ContentType, StatusCode},
    post, put, web, HttpResponse, Responder,
};
use diesel::prelude::*;
use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection, RunQueryDsl};

use crate::{
    models::category::{Category, NewCategory},
    schema::categories,
    services::errors,
};

#[get("")]
pub async fn categories_list(
    pool: web::Data<Pool<AsyncPgConnection>>,
) -> actix_web::Result<impl Responder> {
    let mut connection = pool
        .get()
        .await
        .map_err(|_e| errors::ApiErrors::DatabaseConnectionError)?;

    let db_result = categories::table
        .select(Category::as_select())
        .load(&mut connection)
        .await;
    let categories_vec = db_result.map_err(|_e| errors::ApiErrors::InternalError)?;

    let response_serialized =
        serde_json::to_string(&categories_vec).map_err(|_e| errors::ApiErrors::InternalError)?;

    return Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(response_serialized));
}

pub fn categories_config(cfg: &mut web::ServiceConfig) {
    cfg.service(categories_list);
}
