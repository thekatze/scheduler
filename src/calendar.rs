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
    pub calendar_id: sqlx::types::uuid::Uuid,
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
            li .items-center .justify-center .flex .gap-4 id=(format!("event-{}", self.id)) {
                form action=(format!("{}/{}", self.calendar_id, self.id)) method="POST" {abbr .no-underline title="Delete this event" { button .w-5 .h-5 .font-black .text-xs .pb-1 .text-red-600 type="submit" { "x" } } }
                strong { (self.date.to_string()) }
                (self.summary)
            }
        )
    }
}
