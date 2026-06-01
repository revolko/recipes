use crate::recipes_service::models::recipe::Recipe;
use crate::recipes_service::schema::{categories, recipe_category};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Queryable, Selectable, Identifiable, ToSchema, Serialize, Deserialize, Debug)]
#[diesel(table_name = categories)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(name))]
pub struct Category {
    pub name: String,
}

#[derive(Insertable, ToSchema, Serialize, Deserialize)]
#[diesel(table_name = categories)]
pub struct NewCategory {
    pub name: String,
}

#[derive(AsChangeset, ToSchema, Deserialize)]
#[diesel(table_name = categories)]
pub struct ChangeCategory {
    pub name: Option<String>,
}

#[derive(Identifiable, Queryable, Selectable, Associations, Insertable, Debug)]
#[diesel(table_name = recipe_category)]
#[diesel(belongs_to(Recipe))]
#[diesel(belongs_to(Category, foreign_key = category_name))]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(recipe_id, category_name))]
pub struct RecipeCategory {
    pub recipe_id: i32,
    pub category_name: String,
}
