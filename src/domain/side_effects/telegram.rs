use telegram_bot::Error;
use telegram_bot::Api;

use super::SideEffect;

pub enum TelegramSideEffect {
    MessageRenderer(TelegramMessageRenderer)
}

pub struct TelegramMessageRenderer {
    closure: Box<Fn(&telegram_bot::Api)>,
}

impl std::cmp::PartialEq for TelegramSideEffect {
    fn eq(&self, _other: &TelegramSideEffect) -> bool {
        false
    }
}

impl std::fmt::Debug for TelegramSideEffect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TelegramSideEffect::MessageRenderer { .. } => write!(f, "MessageRenderer()"),
        }
    }
}

impl TelegramMessageRenderer {
    pub fn new<R: telegram_bot::Request + 'static>(request: R) -> Self {
        let closure = move |api: &telegram_bot::Api| {
            api.spawn(&request)
        };

        TelegramMessageRenderer {
            closure: Box::new(closure)
        }
    }
}

impl TelegramMessageRenderer {
    pub fn render(&self, telegram_api: &Api) -> Result<(), Error> {
        (self.closure)(telegram_api);
        Ok(())
    }
}

impl <R: telegram_bot::Request + 'static> From<R> for SideEffect {
    fn from(req: R) -> Self {
        SideEffect::Telegram(
            TelegramSideEffect::MessageRenderer(
                TelegramMessageRenderer::new(req)
            )
        )
    }
}