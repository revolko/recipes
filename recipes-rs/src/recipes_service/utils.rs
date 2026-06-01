use diesel_async::{
    pooled_connection::deadpool::{Object, Pool, PoolError},
    AsyncPgConnection,
};
use std::sync::Arc;

pub async fn get_connection(
    pool: Arc<Pool<AsyncPgConnection>>,
) -> Result<Object<AsyncPgConnection>, PoolError> {
    return pool.get().await;
}
