use diesel::result::Error as DieselError;
use diesel_async::pooled_connection::deadpool::PoolError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Deadpool Pool error: {0}")]
    DbPool(#[from] PoolError),
    #[error("Diesel error : {0}")]
    DbDiesel(#[from] DieselError),
}
