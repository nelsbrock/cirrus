use super::schema::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use getset::Getters;

#[derive(Queryable, Getters)]
pub(crate) struct User {
    name: String,
    password_hash: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub(crate) struct UserInsert<'a> {
    name: &'a str,
    password_hash: &'a str,
}

#[derive(Queryable, Getters)]
pub(crate) struct PublicShare {
    pub id: String,
    pub file_path: String,
    pub created: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = public_shares)]
pub(crate) struct PublicShareInsert<'a> {
    pub id: &'a str,
    pub file_path: &'a str,
}
