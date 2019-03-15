use telegram_bot::{Message};

use crate::domain::context::Context;

use crate::domain::services::{
    users::UserWithRoles,
};
use std::fmt;

pub enum ConfirmationResolverState {
    UserIsNotPresent { context: Context },
    UnsupportedUpdate { context: Context },
    New { context: Context, message: Message, user: UserWithRoles },
    CorrectEmailEntered { context: Context, message: Message, user: UserWithRoles, email: String },
    IncorrectEmailEntered { context: Context, message: Message, user: UserWithRoles },
    CorrectCodeEntered { context: Context, message: Message, user: UserWithRoles, entered_code: String, requested_code: String, email: String },
    IncorrectCodeEntered { context: Context, message: Message, user: UserWithRoles },
}

impl fmt::Display for ConfirmationResolverState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfirmationResolverState::UserIsNotPresent { .. } => write!(f, "UserIsNotPresent()"),
            ConfirmationResolverState::UnsupportedUpdate { .. } => write!(f, "UnsupportedUpdate()"),
            ConfirmationResolverState::New { .. } => write!(f, "New()"),
            ConfirmationResolverState::CorrectEmailEntered { email, .. } => write!(f, "CorrectEmailEntered({})", email),
            ConfirmationResolverState::IncorrectEmailEntered { .. } => write!(f, "IncorrectEmailEntered()"),
            ConfirmationResolverState::CorrectCodeEntered { entered_code, .. } => write!(f, "CorrectCodeEntered({})", entered_code),
            ConfirmationResolverState::IncorrectCodeEntered { .. } => write!(f, "IncorrectCodeEntered()"),
        }
    }
}