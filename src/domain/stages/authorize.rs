use std::sync::Arc;

use crate::domain::context::Context;
use crate::domain::error::PipelineError;
use crate::domain::db::DataBaseSource;
use crate::domain::models;

use crate::lib::pipeline::{Pipeline, PipelineStage};

use telegram_bot::{UpdateKind, Update, User};

use crate::diesel::insert_into;
use crate::diesel::prelude::*;

pub struct AuthorizeStage {
    db: Arc<DataBaseSource>
}

impl AuthorizeStage {
    pub fn new(db: Arc<DataBaseSource>) -> Self {
        AuthorizeStage {
            db
        }
    }

    fn authorize(&self, context: Context, tg_user: User) -> Result<Context, PipelineError> {
        let conn = self.db.get_connection();

        // TODO make it transactional
        self.find_user(&conn, &tg_user).map(|_user| {
            // TODO fill context here
            context
        })
    }

    fn find_user(&self, conn: &PgConnection, tg_user: &User) -> Result<models::User, PipelineError> {
        use crate::schema::users::dsl::*;

        let tg_user_id = get_user_id_from_user(tg_user)
            .map_err(|_err| PipelineError::UnsupportedMessageType {})?;

        users
            .filter(tg_id.eq(tg_user_id))
            .first::<models::User>(conn)
            .optional()
            .and_then(|user| user.map_or_else(
                || self.create_user(conn, tg_user, tg_user_id),
                |user| {
                    info!("User has found: {:?}", user);
                    Ok(user)
                }
            ))
            .map_err(|err| PipelineError::DataBaseError { inner: Box::new(err) })
    }

    fn create_user(&self, conn: &PgConnection, tg_user: &User, tg_user_id: i64) -> Result<models::User, crate::diesel::result::Error> {
        use crate::schema::users::dsl::*;
        insert_into(users)
            .values((
                tg_id.eq(tg_user_id),
                tg_username.eq(&tg_user.username),
                tg_fullname.eq(Some(&tg_user.first_name)),
                active.eq(false)
            ))
            .returning(id)
            .get_result::<i32>(conn)
            .and_then(|new_id| users.filter(id.eq(new_id)).first(conn))
            .map(|new_user| {
                info!("New user has created {:?}", new_user);
                new_user
            })
            .map_err(|err| {
                error!("Cannot create new user: {}", err);
                err
            })
    }
}

impl PipelineStage<Context, PipelineError> for AuthorizeStage {
    fn process(&self, context: Context, next: Arc<Pipeline<Context, PipelineError>>) -> Result<Context, PipelineError> {
        get_user_from_update(&context.update)
            .and_then(|user| self.authorize(context, user))
            .and_then(|ctx| next.call(ctx))
    }
}

fn get_user_id_from_user(user: &User) -> Result<i64, std::num::ParseIntError> {
    format!("{}", user.id).parse()
        .map_err(|err| {
            error!("Cannot fetch id from user {:?}: {}", user, err);
            err
        })
}

fn get_user_from_update(update: &Update) -> Result<User, PipelineError> {
    match &update.kind {
        UpdateKind::Message(message) => Ok(message.from.clone()),
        UpdateKind::EditedMessage(message) => Ok(message.from.clone()),
        UpdateKind::CallbackQuery(message) => Ok(message.from.clone()),
        _ => Err(PipelineError::UnsupportedMessageType {})
    }
}