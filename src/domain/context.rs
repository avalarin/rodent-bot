use crate::lib::telegram::Part;

pub struct Context {
    pub update: telegram_bot::Update,
    pub parts: Vec<Box<Part>>
}

impl Context {
    pub fn new(update: telegram_bot::Update) -> Self {
        Context {
            update,
            parts: vec![]
        }
    }

    pub fn put_part(mut self, part: Box<Part>) -> Self {
        self.parts.push(part);
        self
    }
}
