use serde::Serialize;
use utoipa::ToSchema;

use crate::recipes_service::models::category::Category;

#[derive(Serialize, ToSchema)]
pub struct CategoryResponse {
    pub name: String,
}

#[derive(Serialize, ToSchema)]
pub struct RecipeResponse {
    pub id: i32,
    pub name: String,
    pub instructions: String,
    pub cuisine: String,
    pub duration_min: i32,
    pub preparation_needed: bool,
    pub portions: i32,
    pub difficulty: i32,
    pub categories: Vec<CategoryResponse>,
}

// traits

impl From<Category> for CategoryResponse {
    fn from(category: Category) -> Self {
        Self {
            name: category.name,
        }
    }
}
