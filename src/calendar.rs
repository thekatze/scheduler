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
        html!(
            span {
                strong { (self.date.to_string()) } "-" (self.summary)
            }
        )
    }
}
