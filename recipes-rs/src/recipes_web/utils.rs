use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct ResponseBodyVec<T> {
    pub result: T,
}
