use std::sync::Arc;

use crate::domain::context::Context;
use crate::domain::error::PipelineError;
use crate::domain::resolvers::confirmation::ConfirmationResolver;
use crate::domain::services::{
    users::UserWithRoles
};
use crate::lib::telegram::TelegramUtils;
use crate::lib::pipeline::{Pipeline, PipelineStage};

use telegram_bot::{
    CanReplySendMessage,
    types::Message,
    types::requests::SendMessage
};

pub struct AuthorizeStage {
    resolver: ConfirmationResolver
}

impl PipelineStage<Context, PipelineError> for AuthorizeStage {
    fn process(&self, context: Context, next: Arc<Pipeline<Context, PipelineError>>) -> Result<Context, PipelineError> {
        let user = (&context).user.clone()
            .ok_or_else(|| PipelineError::UserIsRequired {})?;

        match (&user.user.email, user.user.active) {
            // User confirmed his email, call next stage
            (Some(_), true) => next.call(context),

            // User confirmed his email, but disabled, break pipeline
            (Some(_), false) => self.user_disabled(context, user),

            (None, _) => {
                info!("User didn't confirm his email, entering confirmation resolver...");
                self.resolver.process(context)
            },
        }
    }
}


impl AuthorizeStage {
    pub fn new() -> Self {
        AuthorizeStage {
            resolver: ConfirmationResolver::new()
        }
    }

    fn user_disabled(&self, context: Context, user: UserWithRoles) -> Result<Context, PipelineError> {
        info!("User {} is disabled, break execution of pipeline.", user.user.id);
        match TelegramUtils::get_message_from_update(&context.update) {
            // Update hasn't a message, just break execution
            None => Ok(context),
            Some(message) => {
                let resp = Self::resp_user_disabled(message);
                Ok(context.put_part(resp))
            }
        }
    }

    fn resp_user_disabled<'s>(message: Message) -> SendMessage<'s> {
        message.text_reply(
            format!("You are blocked!")
        )
    }

}
