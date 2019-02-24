use std::sync::Arc;

use crate::lib::pipeline::{Pipeline, PipelineBuilder};
use crate::domain::error::Error;
use crate::domain::stages::{LoggingStage, ProcessStage};
use super::context::Context;

pub struct Pipelines {
}

impl Pipelines {
    pub fn create() -> Arc<Pipeline<Context, Error>> {
        PipelineBuilder::new()
            .next_stage(LoggingStage::new())
            .next_stage(ProcessStage::new())
            .build()
    }
}