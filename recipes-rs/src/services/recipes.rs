use diesel::prelude::*;
use diesel_async::{
    pooled_connection::deadpool::Pool,
    AsyncPgConnection, RunQueryDsl
};
use serde::Serialize;
use derive_more::derive::{Display, Error};
use actix_web::{
    get, post, put, delete, web, error,
    http::{StatusCode, header::ContentType},
    HttpResponse, Responder
};

use crate::{models::recipe::{ChangeRecipe, NewRecipe, Recipe}, schema::recipes};

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
    #[display("Not found")]
    NotFound,
}

impl error::ResponseError for ApiErrors {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        match *self {
            // TODO maybe logging here?
            ApiErrors::InternalError => println!("Internal Error"),
            ApiErrors::DatabaseConnectionError => println!("DatabaseConnectionError"),
            ApiErrors::NotFound => println!("Not found"),
        }
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match *self {
            ApiErrors::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiErrors::DatabaseConnectionError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiErrors::NotFound => StatusCode::NOT_FOUND,
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

#[get("/{id}")]
pub async fn recipes_get(
    pool: web::Data<Pool<AsyncPgConnection>>,
    path: web::Path<i32>,
) -> actix_web::Result<impl Responder> {
    use crate::schema::recipes::dsl::*;
    let recipe_id = path.into_inner();

    let mut connection = pool.get().await.map_err(|_e| ApiErrors::DatabaseConnectionError)?;

    let recipe: Recipe = recipes
        .find(recipe_id)
        .first(&mut connection)
        .await
        .map_err(|_e| ApiErrors::NotFound)?;
    let response_serialized = serde_json::to_string(&recipe)
        .map_err(|_e| ApiErrors::InternalError)?;

    return Ok(
        HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(response_serialized)
    );
}

#[post("")]
pub async fn recipes_create(
    pool: web::Data<Pool<AsyncPgConnection>>,
    recipe_body: web::Json<NewRecipe>,
) -> actix_web::Result<impl Responder> {
    use crate::schema::recipes::dsl::*;

    let mut connection = pool.get().await.map_err(|_e| ApiErrors::DatabaseConnectionError)?;

    let recipe: Recipe = diesel::insert_into(recipes)
        .values(&recipe_body.into_inner())
        .returning(Recipe::as_returning())
        .get_result(&mut connection)
        .await
        .map_err(|_e| ApiErrors::InternalError)?;
    let response_serialized = serde_json::to_string(&recipe)
        .map_err(|_e| ApiErrors::InternalError)?;

    return Ok(
        HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(response_serialized)
    );
}

#[put("/{id}")]
pub async fn recipes_change(
    pool: web::Data<Pool<AsyncPgConnection>>,
    path: web::Path<i32>,
    recipe_changeset: web::Json<ChangeRecipe>,
) -> actix_web::Result<impl Responder> {
    use crate::schema::recipes::dsl::*;
    let recipe_id = path.into_inner();
    let recipe_changeset = recipe_changeset.into_inner();

    let mut connection = pool.get().await.map_err(|_e| ApiErrors::DatabaseConnectionError)?;

    let recipe: Recipe = diesel::update(recipes.find(recipe_id))
        .set(&recipe_changeset)
        .returning(Recipe::as_returning())
        .get_result(&mut connection)
        .await
        .map_err(|_e| ApiErrors::InternalError)?;
    let response_serialized = serde_json::to_string(&recipe)
        .map_err(|_e| ApiErrors::InternalError)?;

    return Ok(
        HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(response_serialized)
    );
}

#[delete("/{id}")]
pub async fn recipes_delete(
    pool: web::Data<Pool<AsyncPgConnection>>,
    path: web::Path<i32>,
) -> actix_web::Result<impl Responder> {
    use crate::schema::recipes::dsl::*;
    let recipe_id = path.into_inner();

    let mut connection = pool.get().await.map_err(|_e| ApiErrors::DatabaseConnectionError)?;

    diesel::delete(recipes.find(recipe_id))
        .execute(&mut connection)
        .await
        .map_err(|_e| ApiErrors::InternalError)?;

    return Ok(
        HttpResponse::Ok()
        .content_type(ContentType::json())
        .status(StatusCode::NO_CONTENT)
        .finish()
    );
}

pub fn recipes_config(cfg: &mut web::ServiceConfig) {
    cfg.service(recipes_list);
    cfg.service(recipes_get);
    cfg.service(recipes_create);
    cfg.service(recipes_change);
    cfg.service(recipes_delete);
}
