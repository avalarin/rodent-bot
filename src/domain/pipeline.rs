use std::sync::Arc;

use crate::lib::pipeline::{Pipeline, PipelineBuilder};
use crate::domain::error::Error;
use crate::domain::stages::{logging_stage, process_stage};
use super::context::Context;

pub struct Pipelines {
}

impl Pipelines {
    pub fn create() -> Arc<Pipeline<Context, Error>> {
        PipelineBuilder::new()
            .next(&logging_stage)
            .next(&process_stage)
            .build()
    }
}