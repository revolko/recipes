use diesel::prelude::*;
use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection, RunQueryDsl};
use log::{debug, info};
use std::sync::Arc;

use super::errors::ServiceError;
use super::models::category::{Category, RecipeCategory};
use super::models::image::{Image, NewImage, UpdateImage};
use super::models::ingredient::{Ingredient, NewIngredient, NewRecipeIngredient, RecipeIngredient};
use super::models::recipe::{ChangeRecipe, NewRecipe, Recipe};
use super::schema::categories;
use super::schema::images;
use super::schema::ingredients;
use super::schema::recipe_category;
use super::schema::recipe_ingredient;
use super::schema::recipes;
use super::utils::get_connection;

pub async fn list_recipes(
    db_pool: Arc<Pool<AsyncPgConnection>>,
    category_fitler: &Option<String>,
    cuisine_filter: &Option<String>,
    min_duration: &Option<i32>,
    max_duration: &Option<i32>,
) -> Result<Vec<(Recipe, Vec<Category>)>, ServiceError> {
    info!("Listing recipes");
    let mut connection = get_connection(db_pool).await?;
    let mut all_recipes = recipes::table.select(Recipe::as_select()).into_boxed();

    if let Some(cuisine) = cuisine_filter {
        debug!(cuisine; "Filtering by cuisine");
        all_recipes = all_recipes.filter(recipes::cuisine.eq(cuisine));
    }
    if let Some(category) = category_fitler {
        debug!(category; "Filtering by category");
        let filtered_recipe_ids: Vec<i32> = recipe_category::table
            .select(recipe_category::recipe_id)
            .filter(recipe_category::category_name.eq(category))
            .load(&mut connection)
            .await?;

        all_recipes = all_recipes.filter(recipes::id.eq_any(filtered_recipe_ids));
    }
    if let Some(min_duration) = min_duration {
        debug!(min_duration; "Filtering on minimum duration");
        all_recipes = all_recipes.filter(recipes::duration_min.ge(min_duration));
    }
    if let Some(max_duration) = max_duration {
        debug!(max_duration; "Filtering on maximum duration");
        all_recipes = all_recipes.filter(recipes::duration_min.le(max_duration));
    }
    let all_recipes = all_recipes.load(&mut connection).await?;

    let category_assoc = RecipeCategory::belonging_to(&all_recipes)
        .inner_join(categories::table)
        .select((RecipeCategory::as_select(), Category::as_select()))
        .load(&mut connection)
        .await?;

    return Ok(category_assoc
        .grouped_by(&all_recipes)
        .into_iter()
        .zip(all_recipes)
        .map(|(category_assoc, recipe)| {
            (
                recipe,
                category_assoc
                    .into_iter()
                    .map(|(_, category)| category)
                    .collect(),
            )
        })
        .collect());
}

pub async fn get_recipe(
    db_pool: Arc<Pool<AsyncPgConnection>>,
    recipe_id: &i32,
) -> Result<(Recipe, Vec<Category>), ServiceError> {
    info!(recipe_id; "Getting recipe");
    let mut connection = get_connection(db_pool).await?;
    let recipe = recipes::table
        .select(Recipe::as_select())
        .find(recipe_id)
        .first(&mut connection)
        .await?;

    let categories = RecipeCategory::belonging_to(&recipe)
        .inner_join(categories::table)
        .select(Category::as_select())
        .load(&mut connection)
        .await?;

    return Ok((recipe, categories));
}

pub async fn get_recipe_image(
    db_pool: Arc<Pool<AsyncPgConnection>>,
    recipe_id: &i32,
) -> Result<Image, ServiceError> {
    info!(recipe_id; "Getting recipe image");
    let mut connection = get_connection(db_pool).await?;
    let image = images::table
        .filter(images::recipe_id.eq(recipe_id))
        .select(Image::as_select())
        .first(&mut connection)
        .await?;

    return Ok(image);
}

pub async fn create_recipe(
    db_pool: Arc<Pool<AsyncPgConnection>>,
    new_recipe: &NewRecipe,
    categories_names: &Vec<String>,
    rec_ings: &Vec<NewRecipeIngredient<'_>>,
) -> Result<(Recipe, Vec<Category>), ServiceError> {
    info!(new_recipe:serde, categories:serde = categories_names, ingredients: serde = rec_ings; "Creating recipe");
    let mut connection = get_connection(db_pool).await?;
    // create a recipe, associate it to categories and create and associate ingredients
    // if category does not exists -- fail
    // if ingredient does not exists -- create it
    return connection
        .build_transaction()
        .run(|mut connection| {
            Box::pin(async move {
                let recipe = diesel::insert_into(recipes::table)
                    .values(new_recipe)
                    .returning(Recipe::as_returning())
                    .get_result(&mut connection)
                    .await?;
                debug!(recipe:serde; "Created recipe");

                let mut rec_ings_assoc: Vec<RecipeIngredient> = vec![];
                for rec_ing in rec_ings {
                    let ing: Ingredient = match diesel::insert_into(ingredients::table)
                        .values(&NewIngredient { name: rec_ing.name })
                        .on_conflict_do_nothing()
                        .returning(Ingredient::as_returning())
                        .get_result(&mut connection)
                        .await
                        .optional()?
                    {
                        Some(ingredient) => {
                            debug!(ingredient:serde; "Created ingredient");
                            ingredient
                        }
                        None => {
                            debug!(ingredient = rec_ing.name; "Ingredient already exists");
                            ingredients::table
                                .filter(ingredients::name.eq(rec_ing.name))
                                .select(Ingredient::as_select())
                                .first(&mut connection)
                                .await?
                        }
                    };

                    rec_ings_assoc.push(RecipeIngredient {
                        recipe_id: recipe.id,
                        ingredient_id: ing.id,
                        part: rec_ing.part,
                        quantity: rec_ing.quantity.into(),
                        //TODO: can get rid of to string?
                        unit: rec_ing.unit.to_string(),
                    });
                }
                diesel::insert_into(recipe_ingredient::table)
                    .values(&rec_ings_assoc)
                    .execute(&mut connection)
                    .await?;

                let rec_cats: Vec<RecipeCategory> = categories_names
                    .iter()
                    .map(|category_name| RecipeCategory {
                        recipe_id: recipe.id,
                        category_name: category_name.to_string(),
                    })
                    .collect();
                diesel::insert_into(recipe_category::table)
                    .values(&rec_cats)
                    .execute(&mut connection)
                    .await?;
                debug!(recipe_categories:serde = rec_cats; "Associated categories with recipe");

                let categories = get_recipe_categories(&mut connection, &recipe).await?;
                return Ok((recipe, categories));
            })
        })
        .await;
}

pub async fn update_recipe(
    db_pool: Arc<Pool<AsyncPgConnection>>,
    recipe_id: &i32,
    change_recipe: &ChangeRecipe,
    rec_cats: &Option<Vec<String>>,
) -> Result<(Recipe, Vec<Category>), ServiceError> {
    // add/remove ingredient associations
    info!(recipe_id; "Changing recipe");
    let mut connection = get_connection(db_pool).await?;
    return connection
        .build_transaction()
        .run(|mut connection| {
            Box::pin(async move {
                let recipe = diesel::update(recipes::table.find(recipe_id))
                    .set(change_recipe)
                    .returning(Recipe::as_returning())
                    .get_result(&mut connection)
                    .await?;
                debug!(recipe:serde; "Selected recipe");

                if let Some(rec_cats) = rec_cats {
                    debug!(categories:serde = rec_cats; "Updating categories");
                    // remove rec_cats that are not present in categories and create new ones
                    diesel::delete(
                        RecipeCategory::belonging_to(&recipe)
                            .filter(recipe_category::category_name.ne_all(rec_cats)),
                    )
                    .execute(&mut connection)
                    .await?;

                    let all_rec_cats: Vec<RecipeCategory> = rec_cats
                        .into_iter()
                        .map(|c| RecipeCategory {
                            recipe_id: recipe.id,
                            category_name: c.to_string(),
                        })
                        .collect();
                    diesel::insert_into(recipe_category::table)
                        .values(&all_rec_cats)
                        .on_conflict_do_nothing()
                        .execute(&mut connection)
                        .await?;
                }

                let categories = get_recipe_categories(&mut connection, &recipe).await?;
                return Ok((recipe, categories));
            })
        })
        .await;
}

pub async fn change_recipe_image(
    db_pool: Arc<Pool<AsyncPgConnection>>,
    recipe_id: &i32,
    image_bytes: &'_ Vec<u8>,
    image_type: &mime::Mime,
) -> Result<(), ServiceError> {
    info!(recipe_id; "Changing recipe image");
    let mut connection = get_connection(db_pool).await?;
    let recipe = recipes::table
        .select(Recipe::as_select())
        .find(recipe_id)
        .first(&mut connection)
        .await?;

    diesel::insert_into(images::table)
        .values(&NewImage {
            recipe_id: recipe.id,
            bytes: image_bytes,
            type_: image_type.essence_str(),
        })
        .on_conflict(images::recipe_id)
        .do_update()
        .set(&UpdateImage {
            bytes: image_bytes,
            type_: image_type.essence_str(),
        })
        .execute(&mut connection)
        .await?;
    debug!(recipe_id; "Created/Updated recipe image");

    return Ok(());
}

pub async fn delete_recipe(
    db_pool: Arc<Pool<AsyncPgConnection>>,
    recipe_id: &i32,
) -> Result<(), ServiceError> {
    info!(recipe_id; "Deleting recipe");
    let mut connection = get_connection(db_pool).await?;
    return connection
        .build_transaction()
        .run(|mut connection| {
            Box::pin(async move {
                diesel::delete(
                    recipe_category::table.filter(recipe_category::recipe_id.eq(&recipe_id)),
                )
                .execute(&mut connection)
                .await?;
                debug!("Removed category associations");

                diesel::delete(
                    recipe_ingredient::table.filter(recipe_ingredient::recipe_id.eq(&recipe_id)),
                )
                .execute(&mut connection)
                .await?;
                debug!("Removed ingredient associations");

                diesel::delete(recipes::table.find(&recipe_id))
                    .execute(&mut connection)
                    .await?;
                debug!("Removed recipe");

                return Ok(());
            })
        })
        .await;
}

async fn get_recipe_categories(
    connection: &mut AsyncPgConnection,
    recipe: &Recipe,
) -> Result<Vec<Category>, diesel::result::Error> {
    return RecipeCategory::belonging_to(recipe)
        .inner_join(categories::table)
        .select(Category::as_select())
        .load(connection)
        .await;
}
