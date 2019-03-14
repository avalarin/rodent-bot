use crate::lib::telegram::Part;
use crate::domain::services::users::UserWithRoles;
use crate::domain::services::context::StoredContext;

pub struct Context {
    pub update: telegram_bot::Update,
    pub parts: Vec<Box<Part>>,
    pub user: Option<UserWithRoles>,
    pub stored_context: Option<StoredContext>
}

impl Context {
    pub fn new(update: telegram_bot::Update) -> Self {
        Context {
            update,
            parts: vec![],
            user: None,
            stored_context: None
        }
    }

    pub fn put_part<T: Into<Box<Part>>>(mut self, part: T) -> Self {
        self.parts.push(part.into());
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
