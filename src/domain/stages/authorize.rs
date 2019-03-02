use std::sync::Arc;

use crate::domain::context::Context;
use crate::domain::error::PipelineError;
use crate::domain::services::users::{ UsersService, FindOrCreateUser};
use crate::lib::telegram::TelegramUtils;
use crate::lib::pipeline::{Pipeline, PipelineStage};

use telegram_bot::{User as TgUser};

pub struct AuthorizeStage {
    users: Arc<UsersService>
}

impl AuthorizeStage {
    pub fn new(users: Arc<UsersService>) -> Self {
        AuthorizeStage {
            users
        }
    }

    fn authorize(&self, context: Context, tg_user: TgUser) -> Result<Context, PipelineError> {
        let tg_user_id = TelegramUtils::get_user_id_from_user(&tg_user)
            .ok_or_else(|| PipelineError::UnsupportedMessageType {})?;

        self.users.find_or_create(FindOrCreateUser{
            tg_id: tg_user_id,
            tg_username: tg_user.username,
            tg_fullname: Some(tg_user.first_name)
        }).map(|_user| {
            // TODO fill context here
            context
        }).map_err(PipelineError::from)
    }
}

impl PipelineStage<Context, PipelineError> for AuthorizeStage {
    fn process(&self, context: Context, next: Arc<Pipeline<Context, PipelineError>>) -> Result<Context, PipelineError> {
        TelegramUtils::get_user_from_update(&context.update)
            .ok_or_else(|| PipelineError::UnsupportedMessageType {})
            .and_then(|user| self.authorize(context, user))
            .and_then(|ctx| next.call(ctx))
    }
}