use diesel::pg::data_types::PgTimestamp;

#[derive(Queryable, Debug)]
pub struct Confirmation {
    pub id: i32,
    pub user_id: i32,
    pub email: String,
    pub code: String,
    pub sent_on: PgTimestamp,
    pub expiring_on: PgTimestamp,
    pub confirmed_on: Option<PgTimestamp>,
    pub status: Option<i32>,
}