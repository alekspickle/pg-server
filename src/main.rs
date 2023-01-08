//!  API:
//!  - create-account (The account should consist of details like email, car details and bank details)
//!  - login
//!  - get-details
//!
//! Run with
//!
//! ```not_rust
//! cargo run
//! ```
//!
//! Test with curl:
//! TODO
//! ```not_rust
//! curl 0.0.0.0:3000
//! curl -X POST 0.0.0.0:3000 -d
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
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "docker_pg_setup=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost".to_string());
    info!(db=%db_url, "Using DB url");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_timeout(Duration::from_secs(3))
        .connect(&db_url)
        .await
        .expect("can't connect to database");

    let app = Router::new()
        .route("/create-account", get(handle::create_account))
        .route("/login", post(handle::login))
        .route("/get-details", get(handle::get_details))
        .with_state(pool);

    // Start hyper [`SocketAddr`]
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
