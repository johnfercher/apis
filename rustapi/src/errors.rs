use std::fmt;
use warp::reject::Reject;

#[derive(Debug, Serialize)]
pub struct AppError {
    pub status_code: u16,
    pub message: String,
}

impl AppError {
    pub fn new(message: &str, status_code: StatusCode) -> AppError {
        AppError { message: message.to_string(), status_code: status_code.as_u16() }
    }

    pub fn from_diesel_err(err: diesel::result::Error, context: &str) -> AppError {
        AppError::new(
            format!("{}: {}", context, err.to_string()).as_str(),
            match err {
                diesel::result::Error::DatabaseError(db_err, _) => {
                    match db_err {
                        diesel::result::DatabaseErrorKind::UniqueViolation => StatusCode::BAD_REQUEST,
                        _ => StatusCode::INTERNAL_SERVER_ERROR,
                    }
                }
                diesel::result::Error::NotFound => StatusCode::NOT_FOUND,
                // If needed we can handle other cases
                _ => {
                    StatusCode::INTERNAL_SERVER_ERROR
                }
            },
        )
    }
}

impl Reject for AppError {}

use std::convert::Infallible;
use warp::{Rejection, Reply};
use warp::http::StatusCode;
use serde::{Serialize, Serializer};

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = warp::http::StatusCode::NOT_FOUND.as_u16();
        message = "Not Found";
    } else if let Some(app_err) = err.find::<AppError>() {
        code = app_err.status_code;
        message = app_err.message.as_str();
    } else if let Some(_) = err.find::<warp::filters::body::BodyDeserializeError>() {
        code = warp::http::StatusCode::BAD_REQUEST.as_u16();
        message = "Invalid Body";
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        code = warp::http::StatusCode::METHOD_NOT_ALLOWED.as_u16();
        message = "Method Not Allowed";
    } else {
        eprintln!("unhandled rejection: {:?}", err);
        code = warp::http::StatusCode::INTERNAL_SERVER_ERROR.as_u16();
        message = "Unhandled rejection";
    }

    let json = warp::reply::json(&AppError {
        status_code: code,
        message: message.into(),
    });

    Ok(warp::reply::with_status(json, StatusCode::from_u16(code).unwrap()))
}