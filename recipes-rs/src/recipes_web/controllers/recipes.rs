use actix_web::{
    delete, get,
    http::{header::ContentType, StatusCode},
    post, put, web, HttpResponse, Responder,
};
use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection};
use utoipa_actix_web::service_config;

use crate::recipes_service::models::{
    ingredient::NewRecipeIngredient,
    recipe::{ChangeRecipe as ChangeRecipeUpdate, NewRecipe as NewRecipeInsert, Recipe},
};
use crate::recipes_service::recipes::{
    create_recipe, delete_recipe, get_recipe, list_recipes, update_recipe,
};
use crate::recipes_web::{errors, utils};

use super::{
    requests::recipes::{ChangeRecipe, ListRecipesQuery, NewRecipe},
    responses::json::RecipeResponse,
};

#[utoipa::path(
    tag = "recipes",
    responses(
        (status = 200, description = "List recipes", body = utils::ResponseBodyVec<Vec<RecipeResponse>>)
    )
)]
#[get("")]
pub async fn recipes_list(
    pool: web::Data<Pool<AsyncPgConnection>>,
    query_params: web::Query<ListRecipesQuery>,
) -> actix_web::Result<impl Responder, errors::ApiErrors> {
    // TODO: logging
    let recipes_categories = list_recipes(
        pool.into_inner(),
        &query_params.category,
        &query_params.cuisine,
        &query_params.min_duration,
        &query_params.max_duration,
    )
    .await?;

    let recipes_full: Vec<RecipeResponse> = recipes_categories
        .into_iter()
        .map(|(recipe, categories)| RecipeResponse {
            id: recipe.id,
            name: recipe.name,
            instructions: recipe.instructions,
            cuisine: recipe.cuisine,
            duration_min: recipe.duration_min,
            preparation_needed: recipe.preparation_needed,
            portions: recipe.portions,
            difficulty: recipe.difficulty,
            categories: categories.into_iter().map(|c| c.into()).collect(),
        })
        .collect();

    let response_body = utils::ResponseBodyVec {
        result: recipes_full,
    };
    let response_serialized = serde_json::to_string(&response_body)?;

    return Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(response_serialized));
}

// TODO: add ingredients to response but list should not have categories and ingredients
#[utoipa::path(
    tag = "recipes",
    responses(
        (status = 200, description = "Get recipe", body = RecipeResponse)
    )
)]
#[get("/{id}")]
pub async fn recipes_get(
    pool: web::Data<Pool<AsyncPgConnection>>,
    path: web::Path<i32>,
) -> actix_web::Result<impl Responder, errors::ApiErrors> {
    let recipe_id = path.into_inner();

    let (recipe, categories) = get_recipe(pool.into_inner(), &recipe_id).await?;

    let response = RecipeResponse {
        id: recipe.id,
        name: recipe.name,
        instructions: recipe.instructions,
        cuisine: recipe.cuisine,
        duration_min: recipe.duration_min,
        preparation_needed: recipe.preparation_needed,
        portions: recipe.portions,
        difficulty: recipe.difficulty,
        categories: categories.into_iter().map(|c| c.into()).collect(),
    };

    let response_serialized = serde_json::to_string(&response)?;

    return Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(response_serialized));
}

#[utoipa::path(
    tag = "recipes",
    responses(
        (status = 201, description = "Create recipe", body = Recipe)
    )
)]
#[post("")]
pub async fn recipes_create(
    pool: web::Data<Pool<AsyncPgConnection>>,
    recipe_body: web::Json<NewRecipe>,
) -> actix_web::Result<impl Responder, errors::ApiErrors> {
    let recipe_body = recipe_body.into_inner();
    let new_recipe = NewRecipeInsert {
        name: recipe_body.name,
        instructions: recipe_body.instructions,
        cuisine: recipe_body.cuisine,
        duration_min: recipe_body.duration_min,
        preparation_needed: recipe_body.preparation_needed,
        portions: recipe_body.portions,
        difficulty: recipe_body.difficulty,
    };
    let rec_ings = recipe_body
        .ingredients
        .iter()
        .map(|rec_ing| NewRecipeIngredient {
            name: &rec_ing.name,
            part: rec_ing.part,
            quantity: rec_ing.quantity,
            unit: &rec_ing.unit,
        })
        .collect();

    // TODO: improve error handling -- by improving return of the transaction
    let (recipe, categories) = create_recipe(
        pool.into_inner(),
        &new_recipe,
        &recipe_body.categories,
        &rec_ings,
    )
    .await?;

    let response = RecipeResponse {
        id: recipe.id,
        name: recipe.name,
        instructions: recipe.instructions,
        cuisine: recipe.cuisine,
        duration_min: recipe.duration_min,
        preparation_needed: recipe.preparation_needed,
        portions: recipe.portions,
        difficulty: recipe.difficulty,
        categories: categories.into_iter().map(|c| c.into()).collect(),
    };

    let response_serialized = serde_json::to_string(&response)?;

    return Ok(HttpResponse::Created()
        .content_type(ContentType::json())
        .body(response_serialized));
}

#[utoipa::path(
    tag = "recipes",
    responses(
        (status = 200, description = "Alter recipe", body = RecipeResponse)
    )
)]
#[put("/{id}")]
pub async fn recipes_change(
    pool: web::Data<Pool<AsyncPgConnection>>,
    path: web::Path<i32>,
    recipe_changeset_body: web::Json<ChangeRecipe>,
) -> actix_web::Result<impl Responder, errors::ApiErrors> {
    // TODO: enable update of ingredients
    let recipe_id = path.into_inner();
    let recipe_changeset_body = recipe_changeset_body.into_inner();
    let recipe_changeset = ChangeRecipeUpdate {
        name: recipe_changeset_body.name,
        instructions: recipe_changeset_body.instructions,
        cuisine: recipe_changeset_body.cuisine,
        duration_min: recipe_changeset_body.duration_min,
        preparation_needed: recipe_changeset_body.preparation_needed,
        portions: recipe_changeset_body.portions,
        difficulty: recipe_changeset_body.difficulty,
    };

    let (recipe, categories_vec) = update_recipe(
        pool.into_inner(),
        &recipe_id,
        &recipe_changeset,
        &recipe_changeset_body.categories,
    )
    .await?;
    let recipe_body = RecipeResponse {
        id: recipe.id,
        name: recipe.name,
        instructions: recipe.instructions,
        cuisine: recipe.cuisine,
        duration_min: recipe.duration_min,
        preparation_needed: recipe.preparation_needed,
        portions: recipe.portions,
        difficulty: recipe.difficulty,
        categories: categories_vec.into_iter().map(|c| c.into()).collect(),
    };
    let response_serialized = serde_json::to_string(&recipe_body)?;

    return Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(response_serialized));
}

#[utoipa::path(
    tag = "recipes",
    responses(
        (status = 204, description = "Delete recipe")
    )
)]
#[delete("/{id}")]
pub async fn recipes_delete(
    pool: web::Data<Pool<AsyncPgConnection>>,
    path: web::Path<i32>,
) -> actix_web::Result<impl Responder, errors::ApiErrors> {
    let recipe_id = path.into_inner();

    delete_recipe(pool.into_inner(), &recipe_id).await?;

    return Ok(HttpResponse::NoContent()
        .content_type(ContentType::json())
        .status(StatusCode::NO_CONTENT)
        .finish());
}

pub fn recipes_config(cfg: &mut service_config::ServiceConfig) {
    cfg.service(recipes_list);
    cfg.service(recipes_get);
    cfg.service(recipes_create);
    cfg.service(recipes_change);
    cfg.service(recipes_delete);
}
