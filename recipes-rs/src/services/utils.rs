use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseBodyVec<T> {
    pub result: T,
}
