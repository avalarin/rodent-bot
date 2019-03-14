use telegram_bot::{Message, MessageKind, CanReplySendMessage};
use rand::prelude::*;
use rand::distributions::Alphanumeric;

use crate::lib::telegram::TelegramUtils;
use crate::domain::error::PipelineError;
use crate::domain::context::Context;
use crate::domain::services::context::ConfirmationStoredState;

use super::*;

pub struct ConfirmationResolver {
}

impl ConfirmationResolver {

    pub fn new() -> Self {
        ConfirmationResolver { }
    }

    pub fn process(&self, context: Context) -> Result<Context, PipelineError> {
        let state = self.get_state(context);
        info!("Confirmation resolver is in a state {}", state);
        self.process_state(state)
    }

    fn process_state(&self, state: ConfirmationResolverState) -> Result<Context, PipelineError> {
        match state {
            ConfirmationResolverState::UserIsNotPresent { .. } => Err(PipelineError::UserIsRequired {}),
            ConfirmationResolverState::UnsupportedUpdate { .. } => Err(PipelineError::UnsupportedMessageType {}),

            ConfirmationResolverState::New { context, user, message } => {
                info!("User {} is not confirmed and email is not present. User's email has been requested.", user.user.id);
                Ok(context.put_part(message.text_reply(
                    format!("You have to confirm email. Please, enter your email in response")
                )).replace_stored_context(|stored_context|
                    stored_context.put_confirmation_state(ConfirmationStoredState::requested())
                ))
            },

            ConfirmationResolverState::CorrectEmailEntered { context, user, message, email } => {
                let new_confirmation_code = Self::generate_confirmation_code();

                info!("Email with confirmation code for user {} has been sent to {}", user.user.id, &email);
                info!("TODO DELETE THIS, confirmation code is {}", new_confirmation_code); // TODO send email really
                Ok(context.put_part(message.text_reply(
                    format!("You have been emailed. Please, check your inbox and send confirmation code in response")
                )).replace_stored_context(|stored_context|
                    stored_context.put_confirmation_state(ConfirmationStoredState::sent(new_confirmation_code))
                ))
            },

            ConfirmationResolverState::IncorrectEmailEntered { context, user, message } => {
                info!("User {} has entered a incorrect email in message {:?}", user.user.id, message);
                Ok(context.put_part(message.text_reply(
                    format!("You have entered a incorrect email. Please, check input and try again.")
                )))
            },

            ConfirmationResolverState::CorrectCodeEntered {
                context, user, message,
                entered_code, requested_code
            } => {
                if entered_code == requested_code {
                    info!("User {} has entered a correct confirmation code", user.user.id);
                    Ok(context.put_part(message.text_reply(
                        format!("CONFIRMED")
                    )).replace_stored_context(|stored_context|
                        stored_context.clear_confirmation_state()
                    ))
                } else {
                    info!("User {} has entered a incorrect confirmation code in message {:?}", user.user.id, message);
                    Ok(context.put_part(message.text_reply(
                        format!("You have entered a incorrect confirmation code. Please, check input and try again.")
                    )))
                }
            },

            ConfirmationResolverState::IncorrectCodeEntered { context, user, message  } => {
                info!("User {} has entered a incorrect confirmation code in message {:?}", user.user.id, message);
                Ok(context.put_part(message.text_reply(
                    format!("You have entered a incorrect confirmation code. Please, check input and try again.")
                )))
            }
        }
    }

    fn get_state(&self, context: Context) -> ConfirmationResolverState {
        match ((&context).user.clone(), (&context).stored_context.clone()) {
            // User is not preset in the context
            (None, _) => ConfirmationResolverState::UserIsNotPresent { context },

            // Stored context is not preset in the context
            (_, None) => ConfirmationResolverState::UserIsNotPresent { context },

            // Trying to fetch a message from the update
            (Some(user), Some(stored_context)) => match TelegramUtils::get_message_from_update(&context.update) {
                // Can't fetch a message from the update
                None => ConfirmationResolverState::UnsupportedUpdate { context },

                Some(message) => match stored_context.confirmation_state {
                    None => ConfirmationResolverState::New { context, message, user },

                    Some(ConfirmationStoredState::EmailRequested { .. }) => match Self::get_valid_email_from_message(&message) {
                        // Valid email found
                        Some(email) => ConfirmationResolverState::CorrectEmailEntered { context, message, user, email },

                        // Incorrect email entered
                        None => ConfirmationResolverState::IncorrectEmailEntered { context, message, user },
                    },

                    Some(ConfirmationStoredState::EmailSent { code, .. }) => match Self::get_valid_code_from_message(&message) {
                        // Valid code found
                        Some(entered_code) => ConfirmationResolverState::CorrectCodeEntered { context, message, user, entered_code, requested_code: code },

                        // Incorrect code entered
                        None => ConfirmationResolverState::IncorrectCodeEntered { context, message, user },
                    }
                }
            }
        }
    }

    fn get_valid_email_from_message(message: &Message) -> Option<String> {
        match &message.kind {
            MessageKind::Text { data, .. } if data.contains("@") => Some(data.clone()),
            _ => None
        }
    }

    fn get_valid_code_from_message(message: &Message) -> Option<String> {
        match &message.kind {
            MessageKind::Text { data, .. } => Some(data.clone()),
            _ => None
        }
    }

    fn generate_confirmation_code() -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(4)
            .collect::<String>()
    }
}