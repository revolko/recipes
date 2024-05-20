// @generated automatically by Diesel CLI.

diesel::table! {
    recipes (recipe_id) {
        recipe_id -> Int4,
        name -> Varchar,
        instructions -> Text,
        cuisine -> Varchar,
        duration_min -> Int4,
        preparation_needed -> Bool,
        portions -> Int4,
        difficulty -> Int4,
    }
}
