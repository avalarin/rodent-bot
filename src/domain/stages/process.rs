use std::sync::Arc;

use crate::domain::context::Context;
use crate::domain::error::PipelineError;
use crate::lib::telegram::RequestPart;
use crate::lib::pipeline::{Pipeline, PipelineStage};
use telegram_bot::{UpdateKind, MessageKind, CanReplySendMessage};

pub struct ProcessStage {

}

impl ProcessStage {
    pub fn new() -> Self {
        ProcessStage {}
    }
}

impl PipelineStage<Context, PipelineError> for ProcessStage {
    fn process(&self, context: Context, next: Arc<Pipeline<Context, PipelineError>>) -> Result<Context, PipelineError> {
        if let UpdateKind::Message(message) = &context.update.kind {
            if let MessageKind::Text {ref data, ..} = message.kind {
                let part = RequestPart::new(message.text_reply(
                    format!("Hi, {}! You just wrote '{}'", &message.from.first_name, data)
                ));
                return next.call(context.put_part(Box::new(part)));
            }
        }

        next.call(context)
    }
}