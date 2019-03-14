mod model;
mod service;

pub use self::model::*;
pub use self::service::*;

#[derive(Debug, Fail)]
pub enum ConfirmationServiceError {
    #[fail(display = "db error: {}", inner)]
    DataBaseError {
        inner: Box<diesel::result::Error>,
    }
}

impl From<diesel::result::Error> for ConfirmationServiceError {
    fn from(err: diesel::result::Error) -> ConfirmationServiceError {
        ConfirmationServiceError::DataBaseError { inner: Box::new(err) }
    }
}

pub trait ConfirmationService {
    fn find_latest_confirmation(&self, user_id: i32) -> Result<Option<Confirmation>, ConfirmationServiceError>;

    fn send_confirmation(&self, user_id: i32, email: &String) -> Result<Confirmation, ConfirmationServiceError>;
}