use axum::{
    extract::{Path, State},
    response::{AppendHeaders, IntoResponse, Redirect},
    Form,
};
use maud::{html, Markup};
use serde::Deserialize;
use sqlx::{query, query_as, types::Uuid};

use crate::{
    calendar::{Calendar, Event, Schedule},
    AppContext,
};

use super::layout;

pub(crate) async fn render(
    Path(calendar_id): Path<String>,
    State(context): State<AppContext>,
) -> Markup {
    let calendar_id = Uuid::try_parse(&calendar_id).expect("url path should be valid uuid");

    let calendar_fut = query_as::<_, Calendar>("SELECT * FROM calendars WHERE id = ?1")
        .bind(calendar_id)
        .fetch_one(&context.db);

    let events_fut = query_as::<_, Event>(
        "SELECT * FROM events WHERE calendar_id = ?1 AND date >= current_date ORDER BY date",
    )
    .bind(calendar_id)
    .fetch_all(&context.db);

    let (calendar, events) =
        tokio::try_join!(calendar_fut, events_fut).expect("database queries failed");

    let schedule = Schedule {
        events: events.into_boxed_slice(),
    };

    layout(html!(
        h1 { (calendar.name) }
        a href={(format!("/{}/add", calendar.id))} { "Add event" }
        a href={(format!("/{}/subscription", calendar.id))} { "Calendar Subscription" }
        (schedule)
    ))
}

pub(crate) async fn render_add() -> Markup {
    layout(html!(
        h1 { "Create new Calendar" }
        form method="POST" {
            input type="text" name="name" placeholder="name" required;
            button type="submit" { "Submit" }
        }
    ))
}

#[derive(Debug, Deserialize)]
pub(crate) struct NewCalendarForm {
    name: String,
}
pub(crate) async fn handle_add(
    State(context): State<AppContext>,
    Form(form): Form<NewCalendarForm>,
) -> impl IntoResponse {
    let id = Uuid::new_v4();
    query("INSERT INTO calendars(id, name) VALUES (?1, ?2)")
        .bind(id)
        .bind(form.name)
        .execute(&context.db)
        .await
        .expect("insert failed");

    Redirect::to(&id.hyphenated().to_string())
}

#[axum::debug_handler]
pub(crate) async fn ical_subscription(
    Path(calendar_id): Path<String>,
    State(context): State<AppContext>,
) -> impl IntoResponse {
    let calendar_id = Uuid::try_parse(&calendar_id).expect("url path should be valid uuid");

    let calendar_fut = query_as::<_, Calendar>("SELECT * FROM calendars WHERE id = ?1")
        .bind(calendar_id)
        .fetch_one(&context.db);

    let events_fut = query_as::<_, Event>("SELECT * FROM events WHERE calendar_id = ?1")
        .bind(calendar_id)
        .fetch_all(&context.db);

    let (calendar, events) =
        tokio::try_join!(calendar_fut, events_fut).expect("database query failed");

    use icalendar::{Calendar as ICal, Component as _, Event as ICalEvent, EventLike as _};

    let mut ical_calendar = ICal::new();
    ical_calendar.name(&calendar.name);

    for event in events {
        ical_calendar.push(
            ICalEvent::new()
                .uid(&event.id.hyphenated().to_string())
                .summary(&event.summary)
                .all_day(event.date)
                .done(),
        );
    }

    (
        AppendHeaders([("Content-Type", "text/calendar")]),
        ical_calendar.done().to_string(),
    )
}
