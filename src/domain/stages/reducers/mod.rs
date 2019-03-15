mod telegram;
mod user_actions;

pub use self::telegram::TelegramSideEffectsReducer;
pub use self::user_actions::UserActionsSideEffectsReducer;

use std::sync::Arc;

use crate::domain::side_effects::{
    SideEffect, SideEffectReduceAction
};
use crate::domain::context::Context;
use crate::domain::error::PipelineError;

use crate::lib::pipeline::{Pipeline, PipelineStage};


pub trait SideEffectsReducer {
    fn process(&self, side_effect: &SideEffect) -> SideEffectReduceAction;
}

pub struct SideEffectsReducerStage<T: SideEffectsReducer> {
    reducer: T
}

impl <T: SideEffectsReducer> SideEffectsReducerStage<T> {
    pub fn create(reducer: T) -> SideEffectsReducerStage<T> {
        SideEffectsReducerStage {
            reducer
        }
    }
}

impl <T: SideEffectsReducer> PipelineStage<Context, PipelineError> for SideEffectsReducerStage<T> {
    fn process(&self, context: Context, next: Arc<Pipeline<Context, PipelineError>>) -> Result<Context, PipelineError> {
        next.call(context)
            .map(|next_context| next_context.reduce_side_effects(|se, _| {
                self.reducer.process(se)
            }))
    }
}
