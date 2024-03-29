use thiserror::Error as Error;
use tokio::io::Error as IOError;
use regex::Error as RegexError;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Not found")]
    NotFound,
    
    #[error("Not found: {0}")]
    NotFoundWithCause(String),
    
    #[error("Internal Server Error: {0}")]
    InternalServerError(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("IOError: {0}")]
    IOError(#[from]IOError),

    #[error("Regex Error: {0}")]
    RegexError(#[from]RegexError),
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
            Error::InternalServerError(ref cause) => ErrorBody {
                message: Some("Internal server error".to_string()),
                code: 500.to_string(),
                cause: Some(cause.to_string()),
                payload: None,
            },
            Error::IOError(ref cause) => ErrorBody {
                message: Some("IO Error".to_string()),
                code: 500.to_string(),
                cause: Some(cause.to_string()),
                payload: None,
            },
            Error::RegexError(ref cause) => ErrorBody {
                message: Some("Regex Error".to_string()),
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
