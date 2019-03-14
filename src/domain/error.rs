use failure;

pub use failure::err_msg;

use crate::domain::services::users::UsersServiceError;
use crate::domain::services::context::ContextServiceError;

#[derive(Debug, Fail)]
pub enum PipelineError {
    #[fail(display = "not authorized user: {}", name)]
    NotAuthorized {
        name: String,
    },
    #[fail(display = "unsupported message type")]
    UnsupportedMessageType { },
    #[fail(display = "db error: {}", inner)]
    DataBaseDieselError {
        inner: Box<diesel::result::Error>,
    },
    #[fail(display = "db json error: {}", inner)]
    DataBaseJsonError {
        inner: Box<serde_json::error::Error>,
    },
    #[fail(display = "internal error: user is required")]
    UserIsRequired { },
}

impl From<UsersServiceError> for PipelineError {
    fn from(err: UsersServiceError) -> PipelineError {
        match err {
            UsersServiceError::DataBaseError { inner } => PipelineError::DataBaseDieselError { inner }
        }
    }
}

impl From<ContextServiceError> for PipelineError {
    fn from(err: ContextServiceError) -> PipelineError {
        match err {
            ContextServiceError::DataBaseError { inner } => PipelineError::DataBaseDieselError { inner },
            ContextServiceError::JsonError { inner } => PipelineError::DataBaseJsonError { inner }
        }
    }
}

//macro_rules! err {
//    ($($err:tt)*) => {{
//        error!($($err)*);
//        Err(err_msg(format!($($err)*)))
//    }};
//}
