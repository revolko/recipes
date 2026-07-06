use serde::Serialize;
use utoipa::ToSchema;

use crate::recipes_service::models::category::Category;
use crate::recipes_service::models::ingredient::{Ingredient, RecipeIngredient};

#[derive(Serialize, ToSchema)]
pub struct CategoryResponse {
    pub name: String,
}

#[derive(Serialize, ToSchema)]
pub struct IngredientResponse {
    pub name: String,
    pub part: i16,
    pub quantity: String,
    pub unit: String,
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
    pub ingredients: Vec<IngredientResponse>,
}

// traits

impl From<Category> for CategoryResponse {
    fn from(category: Category) -> Self {
        Self {
            name: category.name,
        }
    }
}

impl From<(RecipeIngredient, Ingredient)> for IngredientResponse {
    fn from(rec_ings: (RecipeIngredient, Ingredient)) -> Self {
        Self {
            name: rec_ings.1.name,
            part: rec_ings.0.part,
            quantity: rec_ings.0.quantity.to_string(),
            unit: rec_ings.0.unit,
        }
    }
}
