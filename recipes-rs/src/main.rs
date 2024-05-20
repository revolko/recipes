pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use schema::recipes;
use std::env;
use dotenvy::dotenv;

use crate::models::{NewRecipe, Recipe};


fn insert_recipe(pg_connection: &mut PgConnection) {
    let new_recipe = NewRecipe {
        name: "recipe",
        instructions: "instructions",
        cuisine: "cuisine",
        duration_min: 123,
        preparation_needed: false,
        portions: 1,
        difficulty: 10,
    };

    diesel::insert_into(recipes::table)
        .values(&new_recipe)
        .returning(Recipe::as_returning())
        .get_result(pg_connection)
        .expect("Error inserting recipe");
}


fn main() {
    dotenv().ok();
    use self::schema::recipes::dsl::*;

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pg_connection = &mut PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    // insert_recipe(pg_connection);

    let result: Vec<Recipe> = recipes
        .select(Recipe::as_select())
        .load(pg_connection)
        .expect("Error loading recipes");

    for recipe in result {
        println!("{}", recipe.recipe_id);
        println!("{}", recipe.name);
    }

    println!("Success");
}
