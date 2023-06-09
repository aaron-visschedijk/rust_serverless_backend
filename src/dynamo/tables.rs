use std::marker::PhantomData;
use super::models::{User, RevokedToken};

pub struct Table <'a, T> {
    pub name: &'a str,
    data_type: PhantomData<T>,
    pub pk: &'a str,
}

// Users table

pub const USER_TABLE: Table<User> = Table::<User> {
    name: "users",
    data_type: PhantomData::<User>,
    pk: "id",
};


pub const REVOKED_TOKENS: Table<RevokedToken> = Table::<RevokedToken>{
    name: "jwt_denylist",
    data_type: PhantomData::<RevokedToken>,
    pk: "jwt",
};