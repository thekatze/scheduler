use axum::{
    extract::{Path, State},
    response::{IntoResponse, Redirect},
    Form,
};
use maud::{html, Markup};
use serde::Deserialize;
use sqlx::{query, query_as, types::Uuid};

use crate::{calendar::Calendar, AppContext};

use super::layout;

pub(crate) async fn render_add() -> Markup {
    layout(html!(
        form method="POST" {
            input type="date" name="date" placeholder="Date";
            input type="text" name="summary" placeholder="Summary";
            button type="submit" { "Submit" }
        }
    ))
}

#[derive(Debug, Deserialize)]
pub(crate) struct AddEventForm {
    date: String,
    summary: String,
}

pub(crate) async fn handle_add(
    Path(calendar_id): Path<String>,
    State(context): State<AppContext>,
    Form(form): Form<AddEventForm>,
) -> impl IntoResponse {
    let calendar_id = Uuid::try_parse(&calendar_id).expect("url path should be valid uuid");

    let calendar = query_as::<_, Calendar>("SELECT * FROM calendars WHERE id = ?1")
        .bind(calendar_id)
        .fetch_one(&context.db)
        .await
        .expect("calendar should exist");

    let id = sqlx::types::uuid::Uuid::new_v4();
    query("INSERT INTO events(calendar_id, id, date, summary) VALUES (?1, ?2, ?3, ?4)")
        .bind(calendar.id)
        .bind(id)
        .bind(form.date)
        .bind(form.summary)
        .execute(&context.db)
        .await
        .expect("create row failed");

    Redirect::to(&format!("/{}", calendar.id))
}
