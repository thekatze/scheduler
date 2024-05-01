use axum::{
    extract::State,
    response::{AppendHeaders, IntoResponse, Redirect},
    Form,
};
use maud::{html, Markup};
use serde::Deserialize;
use sqlx::{query, query_as};

use crate::{
    schedule::{Event, Schedule},
    AppContext,
};

fn layout(page: impl maud::Render) -> Markup {
    html!(
        (maud::DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title { "schedule" }
            }
            body {
                (page)
            }
        }
    )
}

pub(crate) async fn render_index(State(context): State<AppContext>) -> Markup {
    let events =
        query_as::<_, Event>("SELECT * FROM events WHERE date >= current_date ORDER BY date")
            .fetch_all(&context.db)
            .await
            .expect("select failed");

    let schedule = Schedule {
        events: events.into_boxed_slice(),
    };

    layout(schedule)
}

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

#[axum::debug_handler]
pub(crate) async fn handle_post_add(
    State(context): State<AppContext>,
    Form(form): Form<AddEventForm>,
) -> impl IntoResponse {
    let id = sqlx::types::uuid::Uuid::new_v4();
    query("INSERT INTO events(id, date, summary) VALUES (?1, ?2, ?3)")
        .bind(id)
        .bind(form.date)
        .bind(form.summary)
        .execute(&context.db)
        .await
        .expect("create row failed");

    Redirect::to("/")
}

#[axum::debug_handler]
pub(crate) async fn handle_calendar_subscription_get(
    State(context): State<AppContext>,
) -> impl IntoResponse {
    let events = query_as::<_, Event>("SELECT * FROM events")
        .fetch_all(&context.db)
        .await
        .expect("get row failed");

    use icalendar::{Calendar, Component as _, Event as ICalEvent, EventLike as _};

    let mut calendar = Calendar::new();
    calendar.name("Schedule");

    for event in events {
        calendar.push(
            ICalEvent::new()
                .uid(&event.id.hyphenated().to_string())
                .summary(&event.summary)
                .all_day(event.date)
                .done(),
        );
    }

    (
        AppendHeaders([("Content-Type", "text/calendar")]),
        calendar.done().to_string(),
    )
}
