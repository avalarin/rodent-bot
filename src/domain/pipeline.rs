use std::sync::Arc;

use crate::lib::pipeline::{Pipeline, PipelineBuilder};
use crate::domain::db::DataBaseSource;
use crate::domain::error::PipelineError;
use crate::domain::stages::{LoggingStage, ProcessStage, AuthorizeStage, HandleErrorStage};
use super::context::Context;

pub struct Pipelines {
}

impl Pipelines {
    pub fn create(db: Arc<DataBaseSource>) -> Arc<Pipeline<Context, PipelineError>> {
        PipelineBuilder::new()
            .next_stage(HandleErrorStage::new())
            .next_stage(LoggingStage::new())
            .next_stage(AuthorizeStage::new(db))
            .next_stage(ProcessStage::new())
            .build()
    }
}