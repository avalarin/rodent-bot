use std::sync::Arc;

use crate::domain::context::Context;
use crate::domain::error::Error;
use crate::lib::pipeline::Pipeline;
use telegram_bot::{UpdateKind, MessageKind};

pub fn logging_stage(context: Context, next: Arc<Pipeline<Context, Error>>) -> Result<Context, Error> {
    if let UpdateKind::Message(message) = &context.update.kind {
        if let MessageKind::Text {ref data, ..} = message.kind {
            info!("<{}>: {}", &message.from.first_name, data);
        }
    }

    next.call(context)
}