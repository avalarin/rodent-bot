use std::sync::Arc;

use crate::domain::context::Context;
use crate::domain::error::Error;
use crate::lib::pipeline::Pipeline;
use telegram_bot::{UpdateKind, MessageKind, CanReplySendMessage};

pub fn process_stage(context: Context, next: Arc<Pipeline<Context, Error>>) -> Result<Context, Error> {
    if let UpdateKind::Message(message) = &context.update.kind {
        if let MessageKind::Text {ref data, ..} = message.kind {
            context.api.spawn(message.text_reply(
                format!("Hi, {}! You just wrote '{}'", &message.from.first_name, data)
            ));
        }
    }

    next.call(context)
}