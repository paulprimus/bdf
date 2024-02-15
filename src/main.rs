extern crate core;

use std::str::FromStr;
use std::time::Duration;
use axum::Router;
use axum::routing::method_routing::post;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous};
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use error::BdfError;

mod error;
mod authorization;
mod api;
mod db;


#[derive(Debug, Clone)]
pub(crate) struct AppState {
    db: Pool<Sqlite>,
}


#[tokio::main]
async fn main() -> Result<(), BdfError> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "bdf=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    start().await
}

async fn start() -> Result<(), BdfError> {
    let connection = init_db_connection().await;
    let state =  AppState {db: connection};
    let app = router(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .map_err(BdfError::SocketError)?
        ;

    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .await
        .unwrap();
    Ok(())
}

fn router(state: AppState) -> Router {
    Router::new()
        .route("/bdf/api/authorize", post(authorization::authorize))
        .route("/bdf/api/message1", post(api::message1))
        .with_state(state)
        // .layer(prepare_middleware())
}

async fn init_db_connection() -> Pool<Sqlite> {
    info!("init DB Connection!");
    let timeout = Duration::from_secs(5);
    let connection_options = SqliteConnectOptions::from_str("sqlite:bdf.db")
        .expect("Sqliteconnection url nicht valide!")
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal)
        .busy_timeout(timeout);

    SqlitePoolOptions::new()
        .max_connections(5)
        // .connect_timeout(timeout)
        .connect_with(connection_options)
        .await
        .expect("Datenbankverbindung konnte nicht hergestellt werden")
}