use async_graphql::{MergedObject, Object};

mod auth;
use auth::AuthMutation;


mod errors;


#[derive(Default)]
pub struct UsersQuery {}

#[Object]
impl UsersQuery {
    async fn test(&self) -> &'static str {
        "test"
    }
}


#[derive(MergedObject, Default)]
pub struct Query(
    UsersQuery,
);


#[derive(MergedObject, Default)]
pub struct Mutation(
    AuthMutation,
);
