use std::sync::Arc;

use crate::domain::context::Context;
use crate::domain::error::PipelineError;
use crate::domain::services::users::{ UsersService, FindOrCreateUser};
use crate::lib::telegram::TelegramUtils;
use crate::lib::pipeline::{Pipeline, PipelineStage};

use telegram_bot::{User as TgUser};

pub struct IdentifyStage {
    users: Arc<UsersService>
}

impl IdentifyStage {
    pub fn new(users: Arc<UsersService>) -> Self {
        IdentifyStage {
            users
        }
    }

    fn identify(&self, context: Context, tg_user: TgUser) -> Result<Context, PipelineError> {
        let tg_user_id = TelegramUtils::get_user_id_from_user(&tg_user)
            .ok_or_else(|| PipelineError::UnsupportedMessageType {})?;

        self.users.find_or_create(FindOrCreateUser{
            tg_id: tg_user_id,
            tg_username: tg_user.username,
            tg_fullname: Some(tg_user.first_name)
        }).map(|user| {
            context.put_user(user)
        }).map_err(PipelineError::from)
    }
}

impl PipelineStage<Context, PipelineError> for IdentifyStage {
    fn process(&self, context: Context, next: Arc<Pipeline<Context, PipelineError>>) -> Result<Context, PipelineError> {
        TelegramUtils::get_user_from_update(&context.update)
            .ok_or_else(|| PipelineError::UnsupportedMessageType {})
            .and_then(|user| self.identify(context, user))
            .and_then(|ctx| next.call(ctx))
    }
}