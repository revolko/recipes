use actix_web::{
    delete, get,
    http::{header::ContentType, StatusCode},
    post, put, web, HttpResponse, Responder,
};
use diesel::prelude::*;
use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection, RunQueryDsl};

use crate::{
    models::recipe::{ChangeRecipe, NewRecipe, Recipe},
    schema::recipes,
    services::{errors, utils},
};

#[get("")]
pub async fn recipes_list(
    pool: web::Data<Pool<AsyncPgConnection>>,
) -> actix_web::Result<impl Responder> {
    // TODO: logging
    let mut connection = utils::get_connection(pool).await?;

    let db_result = recipes::table
        .select(Recipe::as_select())
        .load(&mut connection)
        .await;
    let recipes_vec = db_result.map_err(|_e| errors::ApiErrors::InternalError)?;

    let response_body = utils::ResponseBodyVec {
        result: recipes_vec,
    };
    let response_serialized =
        serde_json::to_string(&response_body).map_err(|_e| errors::ApiErrors::InternalError)?;

    return Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(response_serialized));
}

#[get("/{id}")]
pub async fn recipes_get(
    pool: web::Data<Pool<AsyncPgConnection>>,
    path: web::Path<i32>,
) -> actix_web::Result<impl Responder> {
    use crate::schema::recipes::dsl::*;
    let recipe_id = path.into_inner();

    let mut connection = utils::get_connection(pool).await?;
    let recipe: Recipe = recipes
        .find(recipe_id)
        .first(&mut connection)
        .await
        .map_err(|_e| errors::ApiErrors::NotFound)?;
    let response_serialized =
        serde_json::to_string(&recipe).map_err(|_e| errors::ApiErrors::InternalError)?;

    return Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(response_serialized));
}

#[post("")]
pub async fn recipes_create(
    pool: web::Data<Pool<AsyncPgConnection>>,
    recipe_body: web::Json<NewRecipe>,
) -> actix_web::Result<impl Responder> {
    use crate::schema::recipes::dsl::*;

    let mut connection = utils::get_connection(pool).await?;

    let recipe: Recipe = diesel::insert_into(recipes)
        .values(&recipe_body.into_inner())
        .returning(Recipe::as_returning())
        .get_result(&mut connection)
        .await
        .map_err(|_e| errors::ApiErrors::InternalError)?;
    let response_serialized =
        serde_json::to_string(&recipe).map_err(|_e| errors::ApiErrors::InternalError)?;

    return Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(response_serialized));
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

    let mut connection = utils::get_connection(pool).await?;

    let recipe: Recipe = diesel::update(recipes.find(recipe_id))
        .set(&recipe_changeset)
        .returning(Recipe::as_returning())
        .get_result(&mut connection)
        .await
        .map_err(|_e| errors::ApiErrors::InternalError)?;
    let response_serialized =
        serde_json::to_string(&recipe).map_err(|_e| errors::ApiErrors::InternalError)?;

    return Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(response_serialized));
}

#[delete("/{id}")]
pub async fn recipes_delete(
    pool: web::Data<Pool<AsyncPgConnection>>,
    path: web::Path<i32>,
) -> actix_web::Result<impl Responder> {
    use crate::schema::recipes::dsl::*;
    let recipe_id = path.into_inner();

    let mut connection = utils::get_connection(pool).await?;

    diesel::delete(recipes.find(recipe_id))
        .execute(&mut connection)
        .await
        .map_err(|_e| errors::ApiErrors::InternalError)?;

    return Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .status(StatusCode::NO_CONTENT)
        .finish());
}

pub fn recipes_config(cfg: &mut web::ServiceConfig) {
    cfg.service(recipes_list);
    cfg.service(recipes_get);
    cfg.service(recipes_create);
    cfg.service(recipes_change);
    cfg.service(recipes_delete);
}
