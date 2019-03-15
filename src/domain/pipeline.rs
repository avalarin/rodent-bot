use std::sync::Arc;

use telegram_bot::Api;

use crate::lib::pipeline::{Pipeline, PipelineBuilder};
use crate::domain::error::PipelineError;
use crate::domain::stages::*;
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
            .next_stage(TelegramSideEffectsStage::new(api))
            .next_stage(UserActionsSideEffectsStage::new(users.clone()))
            .next_stage(HandleErrorStage::new())
            .next_stage(LoggingStage::new())
            .next_stage(IdentifyStage::new(users))
            .next_stage(StoredContextStage::new(context))
            .next_stage(AuthorizeStage::new())
            .next_stage(ProcessStage::new())
            .build()
    }
}