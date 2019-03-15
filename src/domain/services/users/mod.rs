mod service;
mod model;

pub use self::service::*;
pub use self::model::*;

#[derive(Debug, Fail)]
pub enum UsersServiceError {
    #[fail(display = "user {} already confirmed", user_id)]
    UserAlreadyConfirmed { user_id: i32 },
    #[fail(display = "user {} not found", user_id)]
    UserNotFound { user_id: i32 },
    #[fail(display = "db error: {}", inner)]
    DataBaseError {
        inner: Box<diesel::result::Error>,
    }
}

impl From<diesel::result::Error> for UsersServiceError {
    fn from(err: diesel::result::Error) -> UsersServiceError {
        UsersServiceError::DataBaseError { inner: Box::new(err) }
    }
}

pub trait UsersService {
    fn find_or_create(&self, data: FindOrCreateUser) -> Result<UserWithRoles, UsersServiceError>;

    fn confirm_email(&self, user_id: i32, email: String) -> Result<(), UsersServiceError>;
}