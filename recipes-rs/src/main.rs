pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use schema::{recipes, categories, ingredients};
use std::env;
use std::str::FromStr;
use dotenvy::dotenv;
use bigdecimal::{BigDecimal, FromPrimitive};

mod models {
    pub mod recipe;
    pub mod category;
    pub mod ingredient;
}

use models::recipe::{Recipe, NewRecipe};
use models::category::{Category, NewCategory, RecipeCategory};
use models::ingredient::{Ingredient, NewIngredient, RecipeIngredient};


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

    let new_category = NewCategory {
        name: "category",
    };

    diesel::insert_into(categories::table)
        .values(&new_category)
        .execute(pg_connection)
        .expect("Error insertin category");

    let new_ingredient = NewIngredient {
        name: "ingredient",
    };

    diesel::insert_into(ingredients::table)
        .values(&new_ingredient)
        .execute(pg_connection)
        .expect("Error inserting ingredient");
}


fn main() {
    dotenv().ok();
    use self::schema::recipes::dsl::*;
    use self::schema::categories::dsl::*;
    use self::schema::ingredients::dsl::*;
    use self::schema::recipe_category::dsl::recipe_category;

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pg_connection = &mut PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    insert_recipe(pg_connection);

    let mut result_rec: Vec<Recipe> = recipes
        .select(Recipe::as_select())
        .load(pg_connection)
        .expect("Error loading recipes");

    for recipe in &result_rec {
        println!("{}", recipe.id);
        println!("{}", recipe.name);
    }

    let mut result: Vec<Category> = categories
        .select(Category::as_select())
        .load(pg_connection)
        .expect("Error getting categories");

    for category in &result {
        println!("{}", category.id);
        println!("{}", category.name);
    }

    let mut result_ingredients: Vec<Ingredient> = ingredients
        .select(Ingredient::as_select())
        .load(pg_connection)
        .expect("Error getting ingredients");

    let recipe_id = result_rec.pop().expect("Index out of bound").id;
    let category_id = result.pop().expect("Index out of bounds").id;
    let ingredient_id = result_ingredients.pop().expect("Index out of bounds").id;
    let new_recipe_cat = RecipeCategory {
        recipe_id,
        category_id,
    };
    let new_recipe_ingredient = RecipeIngredient {
        recipe_id,
        ingredient_id,
        part: 1,
        quantity: BigDecimal::from_i8(1).unwrap(),
        unit: String::from_str("unit").unwrap(),
    };

    diesel::insert_into(recipe_category)
        .values(&new_recipe_cat)
        .execute(pg_connection)
        .expect("Error creating recipe category");

    println!("Success");
}
