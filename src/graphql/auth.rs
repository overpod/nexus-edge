use ::entity::{user, user::Entity as User};
use async_graphql::{Context, ErrorExtensions, Object, Result};

use nexus_edge::model::{AuthPayload, AuthUser, Language, Role, SignInInput};
use sea_orm::{ColumnTrait, DbConn, EntityTrait, QueryFilter};

use crate::utils::{encryption::verify_password, jwt};

use super::errors::GraphQLError;

#[derive(Default)]
pub struct AuthMutation {}

#[Object]
impl AuthMutation {
    async fn sign_in(&self, ctx: &Context<'_>, input: SignInInput) -> Result<AuthPayload> {
        let db = ctx.data::<DbConn>().unwrap();

        let user = User::find()
            .filter(user::Column::Login.contains(&input.login))
            .one(db)
            .await
            .unwrap();

        if user.is_none() {
            return Err(GraphQLError::UserNotFound.extend());
        }
        let user = user.unwrap();
        let verify = verify_password(&user.password, &input.password);

        if verify.is_err() {
            return Err(GraphQLError::InvalidPassword.extend());
        }

        Ok(AuthPayload {
            token: jwt::sign(user.id.clone())?,
            user: AuthUser {
                id: user.id.to_string(),
                login: user.login,
                role: match user.role {
                    user::Role::Admin => Role::Admin,
                    user::Role::User => Role::User,
                },
                language: match user.language {
                    user::Language::En => Language::En,
                    user::Language::Ru => Language::Ru,
                },
            },
        })
    }
}
