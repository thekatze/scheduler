use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Form,
};
use serde::Deserialize;
use sqlx::{query, query_as, types::Uuid};

use crate::{calendar::Calendar, AppContext};

#[derive(Debug, Deserialize)]
pub(crate) struct AddEventForm {
    date: String,
    summary: String,
}

pub(crate) async fn handle_add(
    Path(calendar_id): Path<String>,
    State(context): State<AppContext>,
    Form(form): Form<AddEventForm>,
) -> Result<impl IntoResponse, StatusCode> {
    let calendar_id = Uuid::try_parse(&calendar_id).map_err(|_| StatusCode::BAD_REQUEST)?;
    let calendar = query_as::<_, Calendar>("SELECT * FROM calendars WHERE id = ?1")
        .bind(calendar_id)
        .fetch_one(&context.db)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let id = sqlx::types::uuid::Uuid::new_v4();
    query("INSERT INTO events(calendar_id, id, date, summary) VALUES (?1, ?2, ?3, ?4)")
        .bind(calendar.id)
        .bind(id)
        .bind(form.date)
        .bind(form.summary)
        .execute(&context.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Redirect::to(&format!("/{}", calendar.id)))
}
