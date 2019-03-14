use std::sync::Arc;

use crate::diesel::insert_into;
use crate::diesel::prelude::*;

use crate::domain::db::DataBaseSource;

use super::*;

pub struct UsersServiceImpl {
    db: Arc<DataBaseSource>
}

impl UsersService for UsersServiceImpl {
    fn find_or_create(&self, data: FindOrCreateUser) -> Result<UserWithRoles, UsersServiceError> {
        let conn = self.db.get_connection();

        conn.transaction::<UserWithRoles, UsersServiceError, _>(|| {
            self.find_user(&conn, &data)
                .and_then(|user| user.map_or_else(
                    || self.create_user(&conn, &data),
                    |user| {
                        info!("User has found: {:?}", user);
                        Ok(user)
                    }
                ))
                .and_then(|user| {
                    self.get_user_roles(user.id)
                        .map(|roles| UserWithRoles { user, roles })
                })
        })
    }
}

impl UsersServiceImpl {
    pub fn new(db: Arc<DataBaseSource>) -> Self {
        UsersServiceImpl { db }
    }

    fn find_user(&self, conn: &PgConnection, data: &FindOrCreateUser) -> Result<Option<User>, UsersServiceError> {
        use crate::schema::users::dsl::*;

        users
            .filter(tg_id.eq(data.tg_id))
            .first::<User>(conn)
            .optional()
            .map_err(|err| UsersServiceError::DataBaseError { inner: Box::new(err) })
    }

    fn create_user(&self, conn: &PgConnection, data: &FindOrCreateUser) -> Result<User, UsersServiceError> {
        use crate::schema::users::dsl::*;

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

    fn get_user_roles(&self, id: i32) -> Result<Vec<String>, UsersServiceError> {
        use crate::schema::*;
        let conn = self.db.get_connection();

        roles::table
            .left_join(user_roles::table.on(
                roles::id.eq(user_roles::role_id)
                    .and(user_roles::user_id.eq(id))
            ))
            .select(roles::name)
            .load::<String>(&conn)
            .map_err(|err| UsersServiceError::DataBaseError { inner: Box::new(err) })
    }
}