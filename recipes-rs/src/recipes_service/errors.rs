use derive_more::{Display, Error};
use diesel::result::Error as DieselError;
use diesel_async::pooled_connection::deadpool::PoolError;

#[derive(Debug, Display, Error)]
pub enum ServiceError {
    DbPool(PoolError),
    DbDiesel(DieselError),
}

impl From<DieselError> for ServiceError {
    fn from(diesel_error: DieselError) -> Self {
        Self::DbDiesel(diesel_error)
    }
}

impl From<PoolError> for ServiceError {
    fn from(pool_error: PoolError) -> Self {
        Self::DbPool(pool_error)
    }
}
