use std::sync::Arc;

use telegram_bot::Api;

use crate::lib::pipeline::{Pipeline, PipelineBuilder};
use crate::domain::error::PipelineError;
use crate::domain::stages::*;
use crate::domain::stages::reducers::*;
use crate::domain::services::{
    users::UsersService,
    context::ContextService
};
use super::context::Context;

pub struct Pipelines {
}

impl Pipelines {
    pub fn create(
        users: Arc<UsersService>,
        context: Arc<ContextService>,
        api: Arc<Api>
    ) -> Arc<Pipeline<Context, PipelineError>> {
        PipelineBuilder::new()
            // side effect reducers
            .next_stage(SideEffectsReducerStage::create(TelegramSideEffectsReducer::new(api)))
            .next_stage(SideEffectsReducerStage::create(UserActionsSideEffectsReducer::new(users.clone())))

            // business logic stages
            .next_stage(HandleErrorStage::new())
            .next_stage(LoggingStage::new())
            .next_stage(IdentifyStage::new(users))
            .next_stage(StoredContextStage::new(context))
            .next_stage(AuthorizeStage::new())
            .next_stage(ProcessStage::new())
            .build()
    }
}