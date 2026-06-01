# recipes-rs

Recipes API in Rust.

# Folder structure

 * src/
    * recipes/ -- DB handling/BI/BL
        * models/ -- DB models/entity mapping
        * recipes.rs
        * caregories.rs
        * ingredients.rs
    * recipes_web/ -- request handling
        * controllers/ -- REST controllers
            * requests/ -- requests sturcts/query or request body mappings
                * recipes.rs -- recipes requests
                * categories.rs -- categories requests
            * responses/ -- response structs (HTML, JSON)
                * json.rs -- JSON-serializeable responses
            * recipes.rs
            * categories.rs
        * endpoints.rs -- definition of API endpoints/actix-web service config (root)
        * errors.rs -- common error handling
