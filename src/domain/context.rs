use crate::lib::telegram::Part;
use crate::domain::services::users::User;

pub struct Context {
    pub update: telegram_bot::Update,
    pub parts: Vec<Box<Part>>,
    pub user: Option<User>
}

impl Context {
    pub fn new(update: telegram_bot::Update) -> Self {
        Context {
            update,
            parts: vec![],
            user: None
        }
    }

    pub fn put_part(mut self, part: Box<Part>) -> Self {
        self.parts.push(part);
        self
    }

    pub fn put_user(mut self, user: User) -> Self {
        self.user = Some(user);
        self
    }
}
