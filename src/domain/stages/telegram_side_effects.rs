use std::sync::Arc;

use telegram_bot::Api;

use crate::domain::context::Context;
use crate::domain::error::PipelineError;
use crate::domain::side_effects::{
    *, SideEffectReduceAction::*
};

use crate::lib::pipeline::{Pipeline, PipelineStage};

#[derive(new)]
pub struct TelegramSideEffectsStage {
    api: Arc<Api>
}

impl PipelineStage<Context, PipelineError> for TelegramSideEffectsStage {
    fn process(&self, context: Context, next: Arc<Pipeline<Context, PipelineError>>) -> Result<Context, PipelineError> {
        next.call(context)
            .map(|next_context| next_context.reduce_side_effects(|se, _| {
                match se {
                    SideEffect::Telegram(telegram_se) => match telegram_se {
                        TelegramSideEffect::MessageRenderer(renderer) => {
                            let _result = renderer.render(&self.api);
                            Pop
                        }
                    },
                    _ => Skip
                }
            }))
    }
}
