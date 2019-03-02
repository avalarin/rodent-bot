use std::sync::Arc;

use crate::diesel::insert_into;
use crate::diesel::prelude::*;
use crate::schema::users::dsl::*;

use crate::domain::db::DataBaseSource;

use super::*;

pub struct UsersServiceImpl {
    db: Arc<DataBaseSource>
}

impl UsersService for UsersServiceImpl {
    fn find_or_create(&self, data: FindOrCreateUser) -> Result<User, UsersServiceError> {
        let conn = self.db.get_connection();

        conn.transaction::<User, UsersServiceError, _>(|| {
            self.find_user(&conn, &data)
                .and_then(|user| user.map_or_else(
                    || self.create_user(&conn, &data),
                    |user| {
                        info!("User has found: {:?}", user);
                        Ok(user)
                    }
                ))
        })
    }
}

impl UsersServiceImpl {
    pub fn new(db: Arc<DataBaseSource>) -> Self {
        UsersServiceImpl { db }
    }

    fn find_user(&self, conn: &PgConnection, data: &FindOrCreateUser) -> Result<Option<User>, UsersServiceError> {
        users
            .filter(tg_id.eq(data.tg_id))
            .first::<User>(conn)
            .optional()
            .map_err(|err| UsersServiceError::DataBaseError { inner: Box::new(err) })
    }

    fn create_user(&self, conn: &PgConnection, data: &FindOrCreateUser) -> Result<User, UsersServiceError> {
        insert_into(users)
            .values((
                tg_id.eq(data.tg_id),
                tg_username.eq(&data.tg_username),
                tg_fullname.eq(&data.tg_fullname),
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
                UsersServiceError::DataBaseError { inner: Box::new(err) }
            })
    }
}