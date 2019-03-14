use telegram_bot::{UpdateKind, Update, Message, User};

pub struct TelegramUtils {

}

impl TelegramUtils {

    pub fn get_message_from_update(update: &Update) -> Option<Message> {
        match &update.kind {
            UpdateKind::Message(message) => Some(message.clone()),
            UpdateKind::EditedMessage(message) => Some(message.clone()),
            _ => None
        }
    }

    pub fn get_user_from_update(update: &Update) -> Option<User> {
        match &update.kind {
            UpdateKind::Message(message) => Some(message.from.clone()),
            UpdateKind::EditedMessage(message) => Some(message.from.clone()),
            UpdateKind::CallbackQuery(message) => Some(message.from.clone()),
            _ => None
        }
    }

    pub fn get_user_id_from_user(user: &User) -> Option<i64> {
        format!("{}", user.id).parse()
            .map_err(|err| {
                error!("Cannot fetch user id from user {:?}: {}", user, err);
                err
            })
            .ok()
    }

}