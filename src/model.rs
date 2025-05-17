use async_graphql::InputObject;
use serde::Deserialize;

#[derive(Debug, Deserialize, InputObject)]
pub struct SignInInput {
    pub username: String,
    pub password: String,
}