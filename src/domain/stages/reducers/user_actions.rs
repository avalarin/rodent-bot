use super::*;

use std::sync::Arc;

use crate::domain::services::users::UsersService;

use crate::domain::side_effects::{
    *, SideEffectReduceAction::*
};

#[derive(new)]
pub struct UserActionsSideEffectsReducer {
    users: Arc<UsersService>
}

impl SideEffectsReducer for UserActionsSideEffectsReducer {
    fn process(&self, side_effect: &SideEffect) -> SideEffectReduceAction {
        match side_effect {
            SideEffect::UserActions(user_se) => match user_se {
                UserActionsSideEffects::ConfirmEmail { user_id, email } => {
                    // TODO send response in case of error
                    let _ = self.users.confirm_email(*user_id, email.clone());
                    Pop
                }
            }
            _ => Skip
        }
    }
}