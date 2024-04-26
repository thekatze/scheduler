use axum::{extract::State, response::{IntoResponse, Redirect}, Form};
use maud::{html, Markup};
use serde::Deserialize;
use sqlx::{query, query_as};

use crate::{
    schedule::{Event, EventType, Schedule},
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
    let events = query_as::<_, Event>("SELECT * FROM events WHERE date >= current_date ORDER BY date")
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
            select name="ty" {
                option selected value="0" {"Exam"}
                option selected value="1" {"Hand-In"}
            }
            button type="submit" { "Submit" }
        }
    ))
}

#[derive(Debug, Deserialize)]
pub(crate) struct AddEventForm {
    date: String,
    ty: EventType,
}
#[axum::debug_handler]
pub(crate) async fn handle_post_add(
    State(context): State<AppContext>,
    Form(form): Form<AddEventForm>,
) -> impl IntoResponse {
    query("INSERT INTO events(date, type) VALUES (?1, ?2)")
        .bind(form.date)
        .bind(form.ty)
        .execute(&context.db)
        .await
        .expect("create row failed");

    Redirect::to("/")
}

