use std::sync::Arc;

use crate::domain::context::Context;
use crate::domain::error::PipelineError;
use crate::lib::pipeline::{Pipeline, PipelineStage};
use crate::lib::telegram::RequestPart;
use telegram_bot::{UpdateKind, CanReplySendMessage};

pub struct HandleErrorStage {
}

impl HandleErrorStage {
    pub fn new() -> Self {
        HandleErrorStage { }
    }
}

impl PipelineStage<Context, PipelineError> for HandleErrorStage {
    fn process(&self, context: Context, next: Arc<Pipeline<Context, PipelineError>>) -> Result<Context, PipelineError> {
        let copy = Context::new(context.update.clone());

        next.call(context).or_else(|error| {
            error!("Error has occurred: {}", error);
            if let UpdateKind::Message(message) = &copy.update.kind {
                let part = RequestPart::new(message.text_reply(
                    format!("Error has occurred: {}", error)
                ));
                Ok(copy.put_part(Box::new(part)))
            } else {
                Ok(copy)
            }
        })
    }
}

