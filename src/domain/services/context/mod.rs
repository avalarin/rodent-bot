mod model;
mod service;

pub use self::model::*;
pub use self::service::*;

#[derive(Debug, Fail)]
pub enum ContextServiceError {
    #[fail(display = "db error: {}", inner)]
    JsonError {
        inner: Box<serde_json::error::Error>,
    },
    #[fail(display = "db error: {}", inner)]
    DataBaseError {
        inner: Box<diesel::result::Error>,
    }
}

impl From<diesel::result::Error> for ContextServiceError {
    fn from(err: diesel::result::Error) -> ContextServiceError {
        ContextServiceError::DataBaseError { inner: Box::new(err) }
    }
}

impl From<serde_json::error::Error> for ContextServiceError {
    fn from(err: serde_json::error::Error) -> ContextServiceError {
        ContextServiceError::JsonError { inner: Box::new(err) }
    }
}

pub trait ContextService {
    fn load_context_for_user(&self, user_id: i32) -> Result<StoredContext, ContextServiceError>;

    fn save_context_for_user(&self, stored_context: &StoredContext, user_id: i32) -> Result<(), ContextServiceError>;
}