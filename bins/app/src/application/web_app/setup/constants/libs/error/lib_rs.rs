pub const CODE: &str = r#"use actix_web::{HttpResponse, ResponseError};
use diesel::result::Error as DieselError;
use r2d2::Error as R2D2Error;
use validr::error::ValidationErrors;
use lettre::transport::smtp::Error as LettreError;
use thiserror::Error;
use serde_json::Error as SerdeJsonError;
use reqwest::Error as ReqError;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Not found")]
    NotFound,

    #[error("Not found: {0}")]
    NotFoundWithCause(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("DB Pool Error: {0}")]
    R2D2Error(#[from] R2D2Error),

    #[error("DB Error: {0}")]
    Diesel(#[from] DieselError),

    #[error("Validation error: {0}")]
    Validation(#[from] ValidationErrors),

    #[error("External API error: {0}")]
    Reqwest(#[from] ReqError),

    #[error("Internal Error {0}")]
    InternalError(String),

    #[error("Internal Server Error: {0}")]
    InternalServerError(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Lettre Error: {0}")]
    Lettre(#[from]LettreError),

    #[error("SerdeJson Error: {0}")]
    SerdeJsonError(#[from]SerdeJsonError),
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        let mut response = match self {
            Error::NotFound => HttpResponse::NotFound(),
            Error::NotFoundWithCause(_) => HttpResponse::NotFound(),
            Error::BadRequest(_) => HttpResponse::BadRequest(),
            Error::Validation(_) => HttpResponse::UnprocessableEntity(),
            Error::R2D2Error(_) => HttpResponse::BadGateway(),

            _ => HttpResponse::InternalServerError(),
        };

        response.json(self.into_error_body())
    }
}


// Helper methods for handling NotFound and Validation variants
impl Error {
    pub fn add_cause_if_not_found(self, cause: &str) -> Error {
        match self {
            Error::NotFound => Error::NotFoundWithCause(cause.to_string()),
            _ => self,
        }
    }

    pub fn is_not_found(&self) -> bool {
        matches!(self, Error::NotFound | Error::NotFoundWithCause(_))
    }

    pub fn is_validation(&self) -> bool {
        matches!(self, Error::Validation(_))
    }

    pub fn into_error_body(&self) -> ErrorBody {
        match *self {
            Error::NotFound => ErrorBody {
                message: Some("Entity not found".to_string()),
                code: 404.to_string(),
                cause: None,
                payload: None,
            },
            Error::NotFoundWithCause(ref cause) => ErrorBody {
                message: Some("Entity not found".to_string()),
                code: 404.to_string(),
                cause: Some(cause.to_string()),
                payload: None,
            },
            Error::BadRequest(ref cause) => ErrorBody {
                message: Some("Bad request".to_string()),
                code: 400.to_string(),
                cause: Some(cause.to_string()),
                payload: None,
            },
            Error::R2D2Error(ref cause) => ErrorBody {
                message: Some("DB Pool Error".to_string()),
                code: "no_connections_available".to_string(),
                cause: Some(cause.to_string()),
                payload: None,
            },
            Error::Diesel(ref cause) => ErrorBody {
                message: Some("DB Error".to_string()),
                code: "db".to_string(),
                cause: Some(cause.to_string()),
                payload: None,
            },
            Error::Validation(ref errors) => ErrorBody {
                message: Some("invalid-body".to_owned()),
                code: 400.to_string(),
                cause: Some("invalid-body".to_owned()),
                payload: Some(serde_json::to_value(errors.get_errors()).unwrap()),
            },
            Error::Reqwest(ref cause) => ErrorBody {
                message: Some("External API error".to_string()),
                code: 500.to_string(),
                cause: Some(cause.to_string()),
                payload: None,
            },
            Error::InternalError(ref cause) => ErrorBody {
                message: Some("Something went wrong".to_string()),
                code: 500.to_string(),
                cause: Some(cause.to_string()),
                payload: None,
            },
            Error::InternalServerError(ref cause) => ErrorBody {
                message: Some("Something went wrong".to_string()),
                code: 500.to_string(),
                cause: Some(cause.to_string()),
                payload: None,
            },
            Error::Unauthorized(ref cause) => ErrorBody {
                message: Some("Unauthorized".to_string()),
                code: 401.to_string(),
                cause: Some(cause.to_string()),
                payload: None,
            },
            Error::Lettre(ref cause) => ErrorBody {
                message: Some("Lettre Error".to_string()),
                code: 500.to_string(),
                cause: Some(cause.to_string()),
                payload: None,
            },
            Error::SerdeJsonError(ref cause) => ErrorBody {
                message: Some("SerdeJson Error".to_string()),
                code: 500.to_string(),
                cause: Some(cause.to_string()),
                payload: None,
            },  
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ErrorBody {
    pub message: Option<String>,
    pub code: String,
    pub cause: Option<String>,
    pub payload: Option<serde_json::Value>,
}

"#;