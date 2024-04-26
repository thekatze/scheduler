use maud::{html, Markup};
use serde_repr::Deserialize_repr;

pub(crate) struct Schedule {
    pub(crate) events: Box<[Event]>,
}

#[derive(sqlx::FromRow)]
pub(crate) struct Event {
    id: i32,
    date: sqlx::types::chrono::NaiveDate,
    #[sqlx(rename = "type")]
    ty: EventType,
}

#[repr(u8)]
#[derive(Debug, sqlx::Type, Deserialize_repr)]
pub(crate) enum EventType {
    #[serde()]
    Exam = 0,
    HandIn = 1,
}

impl maud::Render for Schedule {
    fn render(&self) -> Markup {
        html!(
            ol {
                @for event in self.events.iter() {
                    li id=(format!("event-{}", event.id)) { (event) }
                }
            }
        )
    }
}

impl maud::Render for Event {
    fn render(&self) -> Markup {
        let ty = match self.ty {
            EventType::Exam => "Exam",
            EventType::HandIn => "Hand-In",
        };

        html!(
            span {
                strong { (self.date.to_string()) } (ty)
            }
        )
    }
}
