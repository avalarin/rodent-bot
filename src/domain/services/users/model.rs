#[derive(Queryable, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub tg_id: i64,
    pub tg_username: Option<String>,
    pub tg_fullname: Option<String>,
    pub active: bool,
    pub email: Option<String>
}

#[derive(Debug, Clone)]
pub struct UserWithRoles {
    pub user: User,
    pub roles: Vec<String>
}

pub struct FindOrCreateUser {
    pub tg_id: i64,
    pub tg_username: Option<String>,
    pub tg_fullname: Option<String>
}