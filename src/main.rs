//! ## Overview
//!     Simple PostgreSQL server with simple interaction
//!
//! ## API:
//!  - create-account
//!  - login
//!  - get-details
//!
//! Run with
//!
//! ```bash
//!     cargo run
//!     # or
//!     ./target/debug/pg-server
//!     # or
//!     ./target/release/pg-server
//! ```
//!
//! Test with curl:
//! ```not_rust
//!     curl --location -X POST 'localhost:8080/create-account' \
//!     --header 'Content-Type: application/json' \
//!     --header 'Content-Type: text/plain' \
//!     --data-raw '{
//!         "email": "obi.wan.kenobi@gmail.com",
//!         "car": "Speeder 1625",
//!         "bank": "Naboo National Bank Branch, account <classified>",
//!     }'
//! ```

use axum::{
    routing::{get, post},
    Router,
};
use sqlx::postgres::PgPoolOptions;
use std::{env, net::SocketAddr, time::Duration};
use tracing::{debug, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod handle;

#[tokio::main]
async fn main() {
    tracing_init();

    let db_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost".to_string());
    info!(db=%db_url, "Using DB url");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_timeout(Duration::from_secs(3))
        .connect(&db_url)
        .await
        .expect("can't connect to database");

    // Configure router
    let app = Router::new()
        .route("/", post(handle::login))
        .route("/create-account", get(handle::create_account))
        .route("/get-details", get(handle::get_details))
        .with_state(pool);

    // Start hyper listener
    let addr = SocketAddr::from(([0, 0, 0, 0], 3030));
    debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn tracing_init() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "docker_pg_server=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init()
}
