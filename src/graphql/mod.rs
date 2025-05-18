use async_graphql::{
    Data, MergedObject, MergedSubscription, Object, Response,
    http::{ALL_WEBSOCKET_PROTOCOLS, GraphiQLSource},
};

use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::{State, WebSocketUpgrade},
    http::{HeaderMap, header::AUTHORIZATION},
    response::{Html, IntoResponse, Response as AxumResponse},
};

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
pub struct Query(UsersQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(AuthMutation);

// pub async fn handler(
//         State(AppState {
//         schema,
//         ..
//     }): State<AppState>,
//     headers: HeaderMap, req: GraphQLRequest) -> GraphQLResponse {
//     // let auth = headers.get(AUTHORIZATION);
//     // let auth_header = auth.to_str().unwrap();

//     return schema.execute(req.into_inner()).await.into();
// }
