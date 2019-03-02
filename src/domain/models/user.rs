#[derive(Queryable, Debug)]
pub struct User {
    pub id: i32,
    pub tg_id: i64,
    pub tg_username: Option<String>,
    pub tg_fullname: Option<String>,
    pub active: bool
}