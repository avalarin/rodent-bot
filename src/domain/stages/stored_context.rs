use std::sync::Arc;

use crate::domain::context::Context;
use crate::domain::error::PipelineError;
use crate::domain::services::context::ContextService;
use crate::lib::pipeline::{Pipeline, PipelineStage};

#[derive(new)]
pub struct StoredContextStage {
    context: Arc<ContextService>
}

impl PipelineStage<Context, PipelineError> for StoredContextStage {
    fn process(&self, context: Context, next: Arc<Pipeline<Context, PipelineError>>) -> Result<Context, PipelineError> {
        let user = (&context).user.clone()
            .ok_or_else(|| PipelineError::UserIsRequired {})?;

        self.context.load_context_for_user(user.user.id)
            .map_err(PipelineError::from)
            .and_then(|stored_context| next.call(context.put_stored_context(stored_context)))
            .and_then(|context| {
                context.stored_context.as_ref().map(|sc| {
                    let _ = self.context.save_context_for_user(sc, user.user.id)
                        .map_err(|err| {
                            error!("Cannot update context for user {}: {}", user.user.id, err);
                            err
                        });
                    ()
                });

                Ok(context)
            })
    }
}