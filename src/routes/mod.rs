use maud::{html, Markup};

pub mod index;
pub mod calendar;
pub mod events;

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

