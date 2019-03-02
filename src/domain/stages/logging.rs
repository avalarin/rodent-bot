use std::sync::Arc;

use crate::domain::context::Context;
use crate::domain::error::PipelineError;
use crate::lib::pipeline::{Pipeline, PipelineStage};
use telegram_bot::{UpdateKind, MessageKind};

pub struct LoggingStage {

}

impl LoggingStage {
    pub fn new() -> Self {
        LoggingStage {}
    }
}

impl PipelineStage<Context, PipelineError> for LoggingStage {
    fn process(&self, context: Context, next: Arc<Pipeline<Context, PipelineError>>) -> Result<Context, PipelineError> {
        if let UpdateKind::Message(message) = &context.update.kind {
            if let MessageKind::Text {ref data, ..} = message.kind {
                info!(
                    "Incoming message from {}: {}",
                    &message.from.id,
                    data
                );
            }
        }

        next.call(context)
    }
}