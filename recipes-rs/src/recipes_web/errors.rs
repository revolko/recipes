use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::derive::{Display, Error};

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
