use actix_web::{
    delete, get,
    http::{header::ContentType, StatusCode},
    post, put, web, HttpResponse, Responder,
};
use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection};
use utoipa_actix_web::service_config;

use crate::{
    recipes_service::categories::{
        create_category, delete_category, get_category, list_categories, update_category,
    },
    recipes_service::models::category::{Category, ChangeCategory, NewCategory},
    recipes_web::{errors, utils},
};

use super::responses::json::CategoryResponse;

#[utoipa::path(
    tag = "categories",
    responses(
        (status = 200, description = "List categories", body = utils::ResponseBodyVec<Vec<CategoryResponse>>)
    )
)]
#[get("")]
pub async fn categories_list(
    pool: web::Data<Pool<AsyncPgConnection>>,
) -> actix_web::Result<impl Responder> {
    let categories_db = list_categories(pool.into_inner())
        .await
        .map_err(|_e| errors::ApiErrors::InternalError)?;
    let categories_vec: Vec<CategoryResponse> = categories_db
        .iter()
        .map(|category| CategoryResponse {
            name: category.name.clone(),
        })
        .collect();

    let response_body = utils::ResponseBodyVec {
        result: categories_vec,
    };
    let response_serialized =
        serde_json::to_string(&response_body).map_err(|_e| errors::ApiErrors::InternalError)?;

    return Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(response_serialized));
}

#[utoipa::path(
    tag = "categories",
    responses(
        (status = 200, description = "Get category", body = CategoryResponse)
    )
)]
#[get("/{name}")]
pub async fn categories_get(
    pool: web::Data<Pool<AsyncPgConnection>>,
    path: web::Path<String>,
) -> actix_web::Result<impl Responder> {
    let category_name = path.into_inner();

    let category: Category = get_category(pool.into_inner(), category_name)
        .await
        .map_err(|_e| errors::ApiErrors::NotFound)?;
    let category = CategoryResponse {
        name: category.name,
    };
    let response_serialized =
        serde_json::to_string(&category).map_err(|_e| errors::ApiErrors::InternalError)?;

    return Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(response_serialized));
}

#[utoipa::path(
    tag = "categories",
    responses(
        (status = 201, description = "Create category", body = CategoryResponse)
    )
)]
#[post("")]
pub async fn categories_create(
    pool: web::Data<Pool<AsyncPgConnection>>,
    category_body: web::Json<NewCategory>,
) -> actix_web::Result<impl Responder> {
    let category = create_category(pool.into_inner(), &category_body.into_inner())
        .await
        .map_err(|_e| errors::ApiErrors::InternalError)?;
    let category = CategoryResponse {
        name: category.name,
    };
    let response_serialized =
        serde_json::to_string(&category).map_err(|_e| errors::ApiErrors::InternalError)?;

    return Ok(HttpResponse::Created()
        .content_type(ContentType::json())
        .body(response_serialized));
}

// TODO: is needed??
#[utoipa::path(
    tag = "categories",
    responses(
        (status = 200, description = "Alter category", body = CategoryResponse)
    )
)]
#[put("/{name}")]
pub async fn categories_change(
    pool: web::Data<Pool<AsyncPgConnection>>,
    path: web::Path<String>,
    category_changeset: web::Json<ChangeCategory>,
) -> actix_web::Result<impl Responder> {
    let category_name = path.into_inner();
    let category_changeset = category_changeset.into_inner();

    let category: Category = update_category(pool.into_inner(), category_name, &category_changeset)
        .await
        .map_err(|_e| errors::ApiErrors::InternalError)?;
    let category = CategoryResponse {
        name: category.name,
    };
    let response_serialized =
        serde_json::to_string(&category).map_err(|_e| errors::ApiErrors::InternalError)?;

    return Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(response_serialized));
}

#[utoipa::path(
    tag = "categories",
    responses(
        (status = 200, description = "Delete category")
    )
)]
#[delete("/{name}")]
pub async fn categories_delete(
    pool: web::Data<Pool<AsyncPgConnection>>,
    path: web::Path<String>,
) -> actix_web::Result<impl Responder> {
    let category_name = path.into_inner();

    delete_category(pool.into_inner(), category_name)
        .await
        .map_err(|_e| errors::ApiErrors::BadRequest)?;

    return Ok(HttpResponse::NoContent()
        .content_type(ContentType::json())
        .status(StatusCode::NO_CONTENT)
        .finish());
}

pub fn categories_config(cfg: &mut service_config::ServiceConfig) {
    cfg.service(categories_list);
    cfg.service(categories_get);
    cfg.service(categories_create);
    cfg.service(categories_change);
    cfg.service(categories_delete);
}
