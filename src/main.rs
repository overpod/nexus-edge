use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::GraphQL;
use axum::{Router, routing::get};
use log::info;
use sea_orm::Database;
use std::env;
use tower_http::services::ServeDir;

pub mod graphql;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    env_logger::init();

    let port = env::var("PORT").expect("PORT env var not set");
    let host = env::var("HOST").expect("HOST env var not set");
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL env var not set");

    let db = Database::connect(db_url)
        .await
        .expect("Database connection failed");

    let server_url = format!("{host}:{port}");

    const GPAHQL_ROUTE: &str = "/graphql";

    let schema = Schema::build(graphql::Query, EmptyMutation, EmptySubscription).finish();

    let app = Router::new()
        .route("/", get(|| async { "ok" }))
        .route(
            GPAHQL_ROUTE,
            get(graphql::graphiql_sandbox).post_service(GraphQL::new(schema)),
        )
        .nest_service("/static", ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();

    info!("Start server http://{:}", server_url);
    info!("Graphql sandbox http://{:}{}", server_url, GPAHQL_ROUTE);
    axum::serve(listener, app).await.unwrap();
    db.close().await.unwrap();
    info!("Server stopped");
}
