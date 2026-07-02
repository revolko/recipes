use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use serde::Deserialize;
use utoipa::ToSchema;

// GET

#[derive(Deserialize)]
pub struct ListRecipesQuery {
    pub category: Option<String>,
    pub cuisine: Option<String>,
    pub min_duration: Option<i32>,
    pub max_duration: Option<i32>,
    #[serde(rename = "ingredient")]
    pub ingredients: Option<Vec<String>>,
}

// POST

#[derive(ToSchema, Deserialize)]
pub struct NewIngredients {
    pub name: String,
    pub part: i16,
    pub quantity: i32,
    pub unit: String,
}

#[derive(ToSchema, Deserialize)]
pub struct NewRecipe {
    pub name: String,
    pub instructions: String,
    pub cuisine: String,
    pub duration_min: i32,
    pub preparation_needed: bool,
    pub portions: i32,
    pub difficulty: i32,
    pub categories: Vec<String>,
    pub ingredients: Vec<NewIngredients>,
}

// PUT

#[derive(ToSchema, Deserialize)]
pub struct ChangeRecipe {
    pub name: Option<String>,
    pub instructions: Option<String>,
    pub cuisine: Option<String>,
    pub duration_min: Option<i32>,
    pub preparation_needed: Option<bool>,
    pub portions: Option<i32>,
    pub difficulty: Option<i32>,
    pub categories: Option<Vec<String>>,
}

#[derive(MultipartForm)]
pub struct ChangeRecipeImage {
    #[multipart(limit = "20MB")] // TODO: not sure about the value
    pub image: TempFile,
}
