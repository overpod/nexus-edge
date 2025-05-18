use async_graphql::{EmptySubscription, Enum, Schema};
use async_graphql_axum::{GraphQL, GraphQLRequest, GraphQLResponse};
use axum::{
    extract::State, http::HeaderMap, response::{self, IntoResponse}, routing::get, Router
};
use log::info;
use migration::{Migrator, MigratorTrait};
use nexus_edge::seed;
use sea_orm::{ConnectOptions, Database};
use std::env;
use tower_http::services::ServeDir;

mod utils;

mod graphql;
use crate::graphql::{Mutation, Query};

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum AppErrors {
    #[graphql(name = "USER_NOT_FOUND")]
    UserNotFound,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    env_logger::init();

    let port = env::var("PORT").expect("PORT env var not set");
    let host = env::var("HOST").expect("HOST env var not set");
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL env var not set");

    let mut opt = ConnectOptions::new(db_url);
    opt.sqlx_logging(false);

    let db = Database::connect(opt)
        .await
        .expect("Database connection failed");

    Migrator::up(&db, None).await.unwrap();

    seed::seed(&db).await.expect("Seeding failed");

    let server_url = format!("{host}:{port}");

    const GPAHQL_ROUTE: &str = "/graphql";

    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(db)
        .finish();

    let app = Router::new()
        .route("/", get(|| async { "ok" }))
        .route(
            GPAHQL_ROUTE,
            get(graphiql_sandbox).post_service(GraphQL::new(schema)),
        )
        .nest_service("/static", ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();

    info!("Start server http://{:}", server_url);
    info!("Graphql sandbox http://{:}{}", server_url, GPAHQL_ROUTE);
    axum::serve(listener, app).await.unwrap();
}

async fn graphiql_sandbox() -> impl IntoResponse {
    let html = r##"
                    <!DOCTYPE html>
                                <html>
                                <head>
                                </head>
                                <body>
                                    <div id="sandbox" style="position:absolute;top:0;right:0;bottom:0;left:0"></div>
                                    <script src="/static/embeddable-sandbox.umd.production.min.js"></script>
                                    <script>
                                    new window.EmbeddedSandbox({
                                    target: "#sandbox",
                                    // Pass through your server href if you are embedding on an endpoint.
                                    // Otherwise, you can pass whatever endpoint you want Sandbox to start up with here.
                                    initialEndpoint: window.location.href,
                                    runTelemetry: false
                                    });
                                    </script>         
                                </body>
                            </html>
                    "##;
    return response::Html(html);
}
