use diesel::{query_dsl::methods::SelectDsl, SelectableHelper};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use diesel_async::pooled_connection::deadpool::Pool;
use serde::Serialize;
use derive_more::derive::{Display, Error};
use actix_web::{
    get, web, error,
    http::{StatusCode, header::ContentType},
    HttpResponse, Responder
};

use crate::{models::recipe::Recipe, schema::recipes};

#[derive(Serialize)]
struct ResponseBodyVec<T> {
    pub result: T,
}

#[derive(Debug, Display, Error)]
enum ApiErrors {
    #[display("An internal error occurred. Please try again later.")]
    InternalError,
    #[display("An internal error occurred. Please try again later.")]
    DatabaseConnectionError,
}

impl error::ResponseError for ApiErrors {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        match *self {
            // TODO maybe logging here?
            ApiErrors::InternalError => println!("Internal Error"),
            ApiErrors::DatabaseConnectionError => println!("DatabaseConnectionError"),
        }
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match *self {
            ApiErrors::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiErrors::DatabaseConnectionError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}


#[get("")]
pub async fn recipes_list(
    pool: web::Data<Pool<AsyncPgConnection>>,
) -> actix_web::Result<impl Responder> {
    // TODO: logging
    let mut connection = pool.get().await.map_err(|_e| ApiErrors::DatabaseConnectionError)?;

    let db_result = recipes::table
        .select(Recipe::as_select())
        .load(&mut connection).await;
    let recipes_vec = db_result.map_err(|_e| ApiErrors::InternalError)?;

    let response_body = ResponseBodyVec {
        result: recipes_vec,
    };
    let response_serialized = serde_json::to_string(&response_body)
        .map_err(|_e| ApiErrors::InternalError)?;

    return Ok(
        HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(response_serialized)
    );
}

pub fn recipes_config(cfg: &mut web::ServiceConfig) {
    cfg.service(recipes_list);
}
