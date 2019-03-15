use crate::domain::side_effects::{SideEffect, SideEffectsSet, SideEffectReduceAction};
use crate::domain::services::users::UserWithRoles;
use crate::domain::services::context::StoredContext;

pub struct Context {
    pub update: telegram_bot::Update,
    pub side_effects: SideEffectsSet,
    pub user: Option<UserWithRoles>,
    pub stored_context: Option<StoredContext>
}

impl Context {
    pub fn new(update: telegram_bot::Update) -> Self {
        Context {
            update,
            side_effects: SideEffectsSet::empty(),
            user: None,
            stored_context: None
        }
    }

    pub fn put_side_effect<T: Into<SideEffect>>(mut self, side_effect: T) -> Self {
        self.side_effects = self.side_effects.put(side_effect);
        self
    }

    pub fn reduce_side_effects<F: Fn(&SideEffect, usize) -> SideEffectReduceAction>(mut self, func: F) -> Self {
        self.side_effects = self.side_effects.reduce(func);
        self
    }

    pub fn put_user(mut self, user: UserWithRoles) -> Self {
        self.user = Some(user);
        self
    }

    pub fn put_stored_context(mut self, stored_context: StoredContext) -> Self {
        self.stored_context = Some(stored_context);
        self
    }

    pub fn replace_stored_context<F>(mut self, func: F) -> Self where F : FnOnce(StoredContext) -> StoredContext {
        self.stored_context = Some(func(self.stored_context.unwrap_or_else(StoredContext::empty)));
        self
    }
}
