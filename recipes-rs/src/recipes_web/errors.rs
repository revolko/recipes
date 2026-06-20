use crate::recipes_service::errors::ServiceError;
use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::derive::{Display, Error};
use diesel::result::Error as DieselError;

#[derive(Debug, Display, Error)]
pub enum ApiErrors {
    #[display("An internal error occurred. Please try again later.")]
    InternalError,
    #[display("An internal error occurred. Please try again later.")]
    DatabaseConnectionError,
    #[display("Not found")]
    NotFound,
    #[display("Bad request")]
    BadRequest,
}

impl error::ResponseError for ApiErrors {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        match *self {
            // TODO maybe logging here?
            ApiErrors::InternalError => println!("Internal Error"),
            ApiErrors::DatabaseConnectionError => println!("DatabaseConnectionError"),
            ApiErrors::NotFound => println!("Not found"),
            ApiErrors::BadRequest => println!("Bad reqeust"),
        }
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match *self {
            ApiErrors::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiErrors::DatabaseConnectionError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiErrors::NotFound => StatusCode::NOT_FOUND,
            ApiErrors::BadRequest => StatusCode::BAD_REQUEST,
        }
    }
}

impl From<ServiceError> for ApiErrors {
    fn from(service_error: ServiceError) -> Self {
        match service_error {
            ServiceError::DbDiesel(e) => e.into(),
            _ => Self::InternalError,
        }
    }
}

impl From<DieselError> for ApiErrors {
    fn from(diesel_error: DieselError) -> Self {
        match diesel_error {
            DieselError::NotFound => Self::NotFound,
            _ => Self::InternalError,
        }
    }
}

impl From<serde_json::Error> for ApiErrors {
    fn from(_serde_error: serde_json::Error) -> Self {
        Self::InternalError
    }
}
