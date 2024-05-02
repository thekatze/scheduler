use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{AppendHeaders, IntoResponse, Redirect},
    Form,
};
use maud::html;
use serde::Deserialize;
use sqlx::{query, query_as, types::Uuid};

use crate::{
    calendar::{Calendar, Event, Schedule},
    routes::{header, subheader, submit_button},
    AppContext,
};

use super::layout;

async fn get_calendar_and_events(
    calendar_id: &Uuid,
    context: &AppContext,
) -> Result<(Calendar, Vec<Event>), StatusCode> {
    let calendar_fut = query_as::<_, Calendar>("SELECT * FROM calendars WHERE id = ?1")
        .bind(calendar_id)
        .fetch_one(&context.db);

    let events_fut = query_as::<_, Event>(
        "SELECT * FROM events WHERE calendar_id = ?1 AND date >= current_date ORDER BY date",
    )
    .bind(calendar_id)
    .fetch_all(&context.db);

    let (calendar, events) = tokio::try_join!(calendar_fut, events_fut).map_err(|e| match e {
        sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
        e => {
            tracing::error!("{:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    })?;

    Ok((calendar, events))
}

pub(crate) async fn render(
    Path(calendar_id): Path<String>,
    State(context): State<AppContext>,
) -> Result<impl IntoResponse, StatusCode> {
    let calendar_id = Uuid::try_parse(&calendar_id).map_err(|_| StatusCode::BAD_REQUEST)?;

    let (calendar, events) = get_calendar_and_events(&calendar_id, &context).await?;

    let schedule = Schedule {
        events: events.into_boxed_slice(),
    };

    Ok(layout(html!(
        (header(calendar.name))
        div .flex .flex-col ."md:flex-row" .gap-8 {
            section .flex .flex-col .gap-4 .flex-1 .items-center {
                (subheader("Upcoming Events"))
                (schedule)
            }
            section .flex .flex-col .gap-4 .flex-1 {
                (subheader("Calendar Subscription"))
                p {
                    "Add this calendar as a calendar subscription by using "
                    a .underline href={(format!("/{}/subscription", calendar.id))} { "this link" }
                }
                (subheader("Add Event"))
                form .flex .flex-col .gap-4 action={(format!("/{}/add", calendar.id))} method="POST" {
                    input .px-4 .py-2 .bg-indigo-100 .text-indigo-900 .rounded type="date" name="date" placeholder="Date";
                    input .px-4 .py-2 .bg-indigo-100 .text-indigo-900 .rounded type="text" name="summary" placeholder="Summary";
                    (submit_button("Add Event"))
                }
            }
        }

    )))
}

#[derive(Debug, Deserialize)]
pub(crate) struct NewCalendarForm {
    name: String,
}
pub(crate) async fn handle_new(
    State(context): State<AppContext>,
    Form(form): Form<NewCalendarForm>,
) -> Result<impl IntoResponse, StatusCode> {
    let id = Uuid::new_v4();
    query("INSERT INTO calendars(id, name) VALUES (?1, ?2)")
        .bind(id)
        .bind(form.name)
        .execute(&context.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Redirect::to(&id.to_string()))
}

pub(crate) async fn ical_subscription(
    Path(calendar_id): Path<String>,
    State(context): State<AppContext>,
) -> Result<impl IntoResponse, StatusCode> {
    let calendar_id = Uuid::try_parse(&calendar_id).map_err(|_| StatusCode::BAD_REQUEST)?;
    let (calendar, events) = get_calendar_and_events(&calendar_id, &context).await?;

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

    Ok((
        AppendHeaders([("Content-Type", "text/calendar")]),
        ical_calendar.done().to_string(),
    ))
}
