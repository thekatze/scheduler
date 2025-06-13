use std::env;

use axum::{
    http::StatusCode,
    routing::{get, post},
    Router,
};
use sqlx::{migrate, sqlite::SqliteConnectOptions, SqlitePool};
use tokio::net::TcpListener;

mod calendar;
mod routes;

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

    tracing::info!("Listening on http://{address}");
    let listener = TcpListener::bind(&address)
        .await
        .expect("could not listen to address");

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
        .route("/", get(routes::index::render))
        .route("/favicon.ico", get(StatusCode::NOT_FOUND))
        .route("/new", post(routes::calendar::handle_new))
        .route("/:calendar", get(routes::calendar::render))
        .route("/:calendar/add", post(routes::events::handle_add))
        .route(
            "/:calendar/subscription",
            get(routes::calendar::ical_subscription),
        )
        .route("/:calendar/:event", post(routes::events::handle_delete))
        .with_state(context)
}
