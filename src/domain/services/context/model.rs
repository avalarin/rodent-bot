use chrono::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StoredContext {
    pub confirmation_state: Option<ConfirmationStoredState>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ConfirmationStoredState {
    EmailRequested { requested_at: DateTime<Utc> },
    EmailSent { sent_at: DateTime<Utc>, email: String, code: String, tries: i32 }
}

impl StoredContext {
    pub fn empty() -> StoredContext {
        StoredContext {
            confirmation_state: None
        }
    }

    pub fn put_confirmation_state(mut self, state: ConfirmationStoredState) -> StoredContext {
        self.confirmation_state = Some(state);
        self
    }

    pub fn clear_confirmation_state(mut self) -> StoredContext {
        self.confirmation_state = None;
        self
    }
}

impl ConfirmationStoredState {
    pub fn requested() -> ConfirmationStoredState {
        ConfirmationStoredState::EmailRequested { requested_at: Utc::now() }
    }

    pub fn sent(email: String, code: String) -> ConfirmationStoredState {
        ConfirmationStoredState::EmailSent { email, code, sent_at: Utc::now(), tries: 0 }
    }
}