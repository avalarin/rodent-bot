use std::sync::Arc;

pub struct Context {
    pub api: Arc<telegram_bot::Api>,
    pub update: telegram_bot::Update
}
