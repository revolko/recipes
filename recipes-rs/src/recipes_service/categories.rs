use diesel::prelude::*;
use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection, RunQueryDsl};
use std::sync::Arc;

use super::errors::ServiceError;
use super::models::category::{Category, ChangeCategory, NewCategory, RecipeCategory};
use super::schema::{categories, recipe_category};
use super::utils::get_connection;

pub async fn list_categories(
    db_pool: Arc<Pool<AsyncPgConnection>>,
) -> Result<Vec<Category>, ServiceError> {
    let mut connection = get_connection(db_pool).await?;
    return Ok(categories::table
        .select(Category::as_select())
        .load(&mut connection)
        .await?);
}

pub async fn get_category(
    db_pool: Arc<Pool<AsyncPgConnection>>,
    name: String,
) -> Result<Category, ServiceError> {
    let mut connection = get_connection(db_pool).await?;
    return Ok(categories::table.find(name).first(&mut connection).await?);
}

pub async fn create_category(
    db_pool: Arc<Pool<AsyncPgConnection>>,
    new_category: &NewCategory,
) -> Result<Category, ServiceError> {
    let mut connection = get_connection(db_pool).await?;
    return Ok(diesel::insert_into(categories::table)
        .values(new_category)
        .returning(Category::as_select())
        .get_result(&mut connection)
        .await?);
}

pub async fn update_category(
    db_pool: Arc<Pool<AsyncPgConnection>>,
    name: String,
    change_category: &ChangeCategory,
) -> Result<Category, ServiceError> {
    let mut connection = get_connection(db_pool).await?;
    return Ok(diesel::update(categories::table.find(name))
        .set(change_category)
        .returning(Category::as_select())
        .get_result(&mut connection)
        .await?);
}

pub async fn delete_category(
    db_pool: Arc<Pool<AsyncPgConnection>>,
    name: String,
) -> Result<(), ServiceError> {
    let mut connection = get_connection(db_pool).await?;
    // TODO: return custom error upon rollback
    connection
        .build_transaction()
        .run(|mut connection| {
            Box::pin(async move {
                let recipe_assoc = get_category_recipes(&mut connection, &name).await?;
                if !recipe_assoc.is_empty() {
                    return Err(diesel::result::Error::RollbackTransaction);
                }
                diesel::delete(categories::table.find(name))
                    .execute(&mut connection)
                    .await?;

                return Ok(());
            })
        })
        .await?;

    return Ok(());
}

async fn get_category_recipes(
    connection: &mut AsyncPgConnection,
    name: &str,
) -> Result<Vec<RecipeCategory>, diesel::result::Error> {
    return recipe_category::table
        .filter(recipe_category::category_name.eq(name))
        .load(connection)
        .await;
}
