use super::*;

use std::sync::Arc;

use telegram_bot::Api;

use crate::domain::side_effects::{
    *, SideEffectReduceAction::*
};

#[derive(new)]
pub struct TelegramSideEffectsReducer {
    api: Arc<Api>
}

impl SideEffectsReducer for TelegramSideEffectsReducer {
    fn process(&self, side_effect: &SideEffect) -> SideEffectReduceAction {
        match side_effect {
            SideEffect::Telegram(telegram_se) => match telegram_se {
                TelegramSideEffect::MessageRenderer(renderer) => {
                    let _result = renderer.render(&self.api);
                    Pop
                }
            },
            _ => Skip
        }
    }
}