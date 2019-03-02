use diesel::pg::PgConnection;

pub trait DataBaseSource {
    fn get_connection(&self) -> PgConnection;
}