use diesel::prelude::*;

use super::recipe::Recipe;
use crate::recipes_service::schema::images;

#[derive(Queryable, Identifiable, Selectable, Associations, Debug)]
#[diesel(table_name = images)]
#[diesel(belongs_to(Recipe))]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(id))]
pub struct Image {
    pub id: i32,
    pub bytes: Vec<u8>,
    pub type_: String,
    pub recipe_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = images)]
pub struct NewImage<'a> {
    pub bytes: &'a Vec<u8>,
    pub type_: &'a str,
    pub recipe_id: i32,
}
