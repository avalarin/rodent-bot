use super::SideEffect;

#[derive(Debug, PartialEq)]
pub enum UserActionsSideEffects {
    ConfirmEmail { user_id: i32, email: String }
}

impl From<UserActionsSideEffects> for SideEffect {
    fn from(se: UserActionsSideEffects ) -> Self {
        SideEffect::UserActions(se)
    }
}