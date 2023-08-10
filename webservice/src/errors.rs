use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::{error, Error, HttpResponse};
use serde::Serialize;
use sqlx::error::Error as SQLxError;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Serialize)]
pub enum MyError {
    DBError(String),
    ActixError(String),
    NotFound(String),
}

#[derive(Debug, Serialize)]
pub struct MyErrorResponse {
    error_message: String,
}

impl MyError {
    fn error_response(&self) -> String {
        match self {
            MyError::DBError(msg) => {
                println!("Database error occurred: {:?}", msg);
                "Database error".into()
            }

            MyError::ActixError(msg) => {
                println!("Actix error occurred: {:?}", msg);
                "Internal server error".into()
            }

            MyError::NotFound(msg) => {
                println!("Notfound error occurred: {:?}", msg);
                msg.into()
            }
        }
    }
}

impl error::ResponseError for MyError {
    fn status_code(&self) -> StatusCode {
        match self {
            MyError::DBError(msg) | MyError::ActixError(msg) => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::NotFound(msg) => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code()).json(MyErrorResponse {
            error_message: self.error_response(),
        })
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}

impl From<actix_web::error::Error> for MyError {
    fn from(err: Error) -> Self {
        MyError::ActixError(err.to_string())
    }
}

impl From<SQLxError> for MyError {
    fn from(err: SQLxError) -> Self {
        MyError::DBError(err.to_string())
    }
}
