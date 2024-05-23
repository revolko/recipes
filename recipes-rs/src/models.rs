use diesel::prelude::*;
use crate::schema::recipes;

#[derive(Queryable, Selectable)]
#[diesel(table_name = recipes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Recipe {
    pub id: i32,
    pub name: String,
    pub instructions: String,
    pub cuisine: String,
    pub duration_min: i32,
    pub preparation_needed: bool,
    pub portions: i32,
    pub difficulty: i32,
}

#[derive(Insertable)]
#[diesel(table_name = recipes)]
pub struct NewRecipe<'a> {
    pub name: &'a str,
    pub instructions: &'a str,
    pub cuisine: &'a str,
    pub duration_min: i32,
    pub preparation_needed: bool,
    pub portions: i32,
    pub difficulty: i32,
}
