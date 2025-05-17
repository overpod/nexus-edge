use async_graphql::{Context, Object, Result, ErrorExtensions};
use ::entity::{user, user::Entity as User};

use nexus_edge::model::SignInInput;
use sea_orm::{DbConn, EntityTrait, QueryFilter, ColumnTrait};

use crate::utils::encryption::verify_password;

use super::errors::GraphQLError;

#[derive(Default)]
pub struct AuthMutation {}

#[Object]
impl AuthMutation {
    async fn sign_in(&self, ctx: &Context<'_>, input: SignInInput) -> Result<bool> {
        let db = ctx.data::<DbConn>()?;
        let user = User::find()
            .filter(user::Column::Name.eq(input.username))
            .one(db)
            .await
            .unwrap();

        if user.is_none() {
            return Err(GraphQLError::UserNotFound.extend())
        }
        let user = user.unwrap();
        verify_password(&user.password, &input.password)?;

        //let password_hash = &user.password_hash.ok_or(ApiError::AccessDenied)?;
        // let model_input = nexus_edge::model::SignInInput {
        //     username: input.username,
        //     password: input.password,
        // };
        // let user = service::sign_in(db.clone(), model_input).await;

        Ok(true)
    }
}
