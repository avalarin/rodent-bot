use std::sync::Arc;

use crate::domain::context::Context;
use crate::domain::error::PipelineError;
use crate::domain::services::users::UsersService;
use crate::domain::side_effects::{
    *, SideEffectReduceAction::*
};

use crate::lib::pipeline::{Pipeline, PipelineStage};

#[derive(new)]
pub struct UserActionsSideEffectsStage {
    users: Arc<UsersService>
}

impl PipelineStage<Context, PipelineError> for UserActionsSideEffectsStage {
    fn process(&self, context: Context, next: Arc<Pipeline<Context, PipelineError>>) -> Result<Context, PipelineError> {
        next.call(context)
            .map(|next_context| next_context.reduce_side_effects(|se, _| {
                match se {
                    SideEffect::UserActions(user_se) => match user_se {
                        UserActionsSideEffects::ConfirmEmail { user_id, email } => {
                            // TODO send response in case of error
                            let _ = self.users.confirm_email(*user_id, email.clone());
                            Pop
                        }
                    }
                    _ => Skip
                }
            }))
    }
}
