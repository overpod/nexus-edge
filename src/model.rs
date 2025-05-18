use async_graphql::{Enum, InputObject, SimpleObject};
use serde::Deserialize;

#[derive(Debug, Deserialize, InputObject)]
pub struct SignInInput {
    pub login: String,
    pub password: String,
}

#[derive(Debug, Enum, Copy, Clone, Eq, PartialEq)]
pub enum Role {
    #[graphql(name="ADMIN")]
    Admin,
    #[graphql(name="USER")]
    User,
}

#[derive(Debug, Enum, Copy, Clone, Eq, PartialEq)]
pub enum Language {
    #[graphql(name="EN")]
    En,
    #[graphql(name="RU")]
    Ru,
}

#[derive(Debug, SimpleObject)]
pub struct AuthUser {
    pub id: String,
    pub login: String,
    pub role: Role,
    pub language: Language,
}


#[derive(Debug, SimpleObject)]
pub struct AuthPayload {
    pub token: String,
    pub user: AuthUser,
}