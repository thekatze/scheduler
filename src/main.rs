use std::env;

use axum::{routing::get, Router};
use sqlx::{migrate, sqlite::SqliteConnectOptions, SqlitePool};
use tokio::net::TcpListener;

mod routes;
mod schedule;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let db_file_name = env::var("SCHEDULE_DB_FILE").unwrap_or(":memory:".into());
    let address = env::var("SCHEDULE_ADDRESS").unwrap_or("0.0.0.0:80".into());

    let db = SqlitePool::connect_with(
        SqliteConnectOptions::new()
            .filename(db_file_name)
            .create_if_missing(true),
    )
    .await
    .expect("database connection failed");

    tracing::info!("Applying database migrations");
    migrate!("./migrations")
        .run(&db)
        .await
        .expect("migrations failed");

    tracing::info!("Building app");
    let app = build_app(db);

    let listener = TcpListener::bind(&address)
        .await
        .expect("could not listen to port 5173");

    tracing::info!("Listening on {address}");
    axum::serve(listener, app.into_make_service())
        .await
        .expect("server crashed :(")
}

#[derive(Clone)]
pub(crate) struct AppContext {
    db: SqlitePool,
}

fn build_app(db: SqlitePool) -> Router {
    let context = AppContext { db };
    Router::new()
        .route("/", get(routes::render_index))
        .route(
            "/add",
            get(routes::render_add).post(routes::handle_post_add),
        )
        .with_state(context)
}