use failure;

pub use failure::err_msg;

use crate::domain::services::users::UsersServiceError;

#[derive(Debug, Fail)]
pub enum PipelineError {
    #[fail(display = "not authorized user: {}", name)]
    NotAuthorized {
        name: String,
    },
    #[fail(display = "unsupported message type")]
    UnsupportedMessageType { },
    #[fail(display = "db error: {}", inner)]
    DataBaseError {
        inner: Box<diesel::result::Error>,
    }
}

impl From<UsersServiceError> for PipelineError {
    fn from(err: UsersServiceError) -> PipelineError {
        match err {
            UsersServiceError::DataBaseError { inner } => PipelineError::DataBaseError { inner }
        }
    }
}

//macro_rules! err {
//    ($($err:tt)*) => {{
//        error!($($err)*);
//        Err(err_msg(format!($($err)*)))
//    }};
//}
