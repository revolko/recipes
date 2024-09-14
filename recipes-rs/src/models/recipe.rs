use diesel::{Queryable, Selectable, Insertable};
use serde::{Deserialize, Serialize};
use crate::schema::recipes;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = recipes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Recipe {
    pub id: i32,  // TODO: should be u32
    pub name: String,
    pub instructions: String,
    pub cuisine: String,
    pub duration_min: i32,
    pub preparation_needed: bool,
    pub portions: i32,
    pub difficulty: i32,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = recipes)]
pub struct NewRecipe {
    pub name: String,
    pub instructions: String,
    pub cuisine: String,
    pub duration_min: i32,
    pub preparation_needed: bool,
    pub portions: i32,
    pub difficulty: i32,
}
