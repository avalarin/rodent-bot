use std::sync::Arc;
use std::time::SystemTime;

use crate::diesel::insert_into;
use crate::diesel::prelude::*;

use crate::domain::db::DataBaseSource;

use super::*;

pub struct ContextServiceImpl {
    db: Arc<DataBaseSource>
}

impl ContextServiceImpl {

    pub fn new(db: Arc<DataBaseSource>) -> Self {
        ContextServiceImpl {
            db
        }
    }

}

impl ContextService for ContextServiceImpl {

    fn load_context_for_user(&self, user_id: i32) -> Result<StoredContext, ContextServiceError> {
        use crate::schema::stored_contexts;
        let conn = self.db.get_connection();

        stored_contexts::table
            .filter(stored_contexts::user_id.eq(user_id))
            .select(stored_contexts::data)
            .first::<serde_json::Value>(&conn)
            .optional()
            .map_err(From::from)
            .map(|json_optional| {
                json_optional
                    .map(|json| serde_json::from_value(json)
                        .unwrap_or_else(|err| {
                            warn!("Cannot deserialize stored_context for user {}: {}", user_id, err);
                            StoredContext::empty()
                        })
                    )
                    .unwrap_or_else(StoredContext::empty)
            })
    }

    fn save_context_for_user(&self, stored_context: &StoredContext, user_id: i32) -> Result<(), ContextServiceError> {
        use crate::schema::stored_contexts;
        let conn = self.db.get_connection();

        serde_json::to_value(stored_context)
            .map_err(From::from)
            .and_then(|json| {
                insert_into(stored_contexts::table)
                    .values((
                        stored_contexts::user_id.eq(user_id),
                        stored_contexts::updated_on.eq(SystemTime::now()),
                        stored_contexts::data.eq(&json)
                    ))
                    .on_conflict(stored_contexts::user_id)
                    .do_update()
                    .set((
                        stored_contexts::updated_on.eq(SystemTime::now()),
                        stored_contexts::data.eq(&json)
                    ))
                    .execute(&conn)
                    .map(|_| ())
                    .map_err(From::from)
            })
    }
}