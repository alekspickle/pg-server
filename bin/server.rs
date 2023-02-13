use axum::{
    routing::{get, post},
    Router,
};
use pg_server::{data, handle, DEFAULT_DB};
use sqlx::postgres::PgPoolOptions;
use std::{env, net};
use tracing::{debug, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_init();

    let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| DEFAULT_DB.to_string());
    debug!(db=%db_url, "Using DB url");

    let pool = PgPoolOptions::new().connect(&db_url).await?;
    debug!(pool=?pool, "Pool");

    // Create table
    sqlx::query(&data::queries::create_table())
        .execute(&pool)
        .await?;

    // Configure router
    let app = Router::new()
        .route("/", post(handle::login))
        .route("/create-account", get(handle::create_account))
        .route("/get-details/:email", get(handle::get_details))
        .with_state(pool);

    // Start hyper listener
    let addr = net::SocketAddr::from(([0, 0, 0, 0], 3030));
    info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .map_err(Into::into)
}
