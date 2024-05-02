use maud::{html, Markup};

#[derive(sqlx::FromRow)]
pub(crate) struct Calendar {
    pub id: sqlx::types::uuid::Uuid,
    pub(crate) name: String,
}

pub(crate) struct Schedule {
    pub(crate) events: Box<[Event]>,
}

#[derive(sqlx::FromRow)]
pub(crate) struct Event {
    pub id: sqlx::types::uuid::Uuid,
    pub date: sqlx::types::chrono::NaiveDate,
    pub summary: String,
}

impl maud::Render for Schedule {
    fn render(&self) -> Markup {
        let has_events = !self.events.is_empty();
        html!(
            ol .flex .flex-col .gap-4 .p-4 {
                @if has_events {
                    @for event in self.events.iter() {
                        (event)
                    }
                } @else {
                    span .text-indigo-400 {"No upcoming events"}
                }
            }
        )
    }
}

impl maud::Render for Event {
    fn render(&self) -> Markup {
        html!(
            li .flex .gap-4 id=(format!("event-{}", self.id)) {
                strong { (self.date.to_string()) } (self.summary)
            }
        )
    }
}
