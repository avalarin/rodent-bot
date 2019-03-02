use diesel::prelude::*;
use diesel::pg::PgConnection;

use crate::domain::db::DataBaseSource;
use crate::domain::config::PostgresConfiguration;

pub struct DB {
    config: PostgresConfiguration
}

impl DB {
    pub fn new(config: PostgresConfiguration) -> Self {
        DB {
            config
        }
    }
}

impl DataBaseSource for DB {
    fn get_connection(&self) -> PgConnection {
        PgConnection::establish(&self.config.url)
            .expect(&format!("Error connecting to {}", &self.config.url))
    }
}