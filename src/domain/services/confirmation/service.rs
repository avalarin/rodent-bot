use std::sync::Arc;
use std::time::SystemTime;

use rand::prelude::*;
use rand::distributions::Alphanumeric;

use crate::diesel::insert_into;
use crate::diesel::prelude::*;

use crate::domain::db::DataBaseSource;

use super::*;

pub struct ConfirmationServiceImpl {
    db: Arc<DataBaseSource>
}

impl super::ConfirmationService for ConfirmationServiceImpl {
    fn find_latest_confirmation(&self, user_id: i32) -> Result<Option<Confirmation>, ConfirmationServiceError> {
        use crate::schema::confirmations;
        let conn = self.db.get_connection();

        confirmations::table
            .filter(confirmations::user_id.eq(user_id))
            .order(confirmations::sent_on.desc())
            .first::<Confirmation>(&conn)
            .optional()
            .map_err(|err| ConfirmationServiceError::DataBaseError { inner: Box::new(err) })

    }

    fn send_confirmation(&self, user_id: i32, email: &String) -> Result<Confirmation, ConfirmationServiceError> {
        use crate::schema::confirmations;
        let conn = self.db.get_connection();

        let code = self.generate_code();

        insert_into(confirmations::table)
            .values((
                confirmations::user_id.eq(user_id),
                confirmations::email.eq(email),
                confirmations::code.eq(code),
                confirmations::expiring_on.eq(SystemTime::now()),
                confirmations::status.eq(1)
            ))
            .returning(confirmations::id)
            .get_result::<i32>(&conn)
            .and_then(|new_id| confirmations::table.filter(confirmations::id.eq(new_id)).first::<Confirmation>(&conn))
            .map(|new_confirmation| {
                info!("New confirmation #{} has been created", new_confirmation.id);
                new_confirmation
            })
            .map_err(|err| {
                error!("Cannot create new confirmation: {}", err);
                ConfirmationServiceError::DataBaseError { inner: Box::new(err) }
            })
    }
}

impl ConfirmationServiceImpl {
    pub fn new(db: Arc<DataBaseSource>) -> Self {
        ConfirmationServiceImpl { db }
    }

    fn generate_code(&self) -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .collect::<String>()
    }
}

