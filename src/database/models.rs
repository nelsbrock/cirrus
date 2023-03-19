use super::schema::*;
use crate::util;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use getset::Getters;
use std::borrow::Cow;

#[derive(Queryable, Getters)]
#[getset(get = "pub")]
pub struct User {
    name: String,
    password_hash: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct UserInsert<'a> {
    name: Cow<'a, str>,
    password_hash: Cow<'a, str>,
}

impl<'a> UserInsert<'a> {
    pub fn new(name: Cow<'a, str>, password_hash: Cow<'a, str>) -> Self {
        Self {
            name,
            password_hash,
        }
    }

    pub fn new_with_password(name: Cow<'a, str>, password: &[u8]) -> anyhow::Result<Self> {
        Ok(UserInsert::new(
            name,
            Cow::Owned(util::hash_password(password)?),
        ))
    }
}

#[derive(Queryable, Getters)]
#[getset(get = "pub")]
pub struct PublicShare {
    id: String,
    file_path: String,
    created: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = public_shares)]
pub struct PublicShareInsert<'a> {
    id: &'a str,
    file_path: &'a str,
}
