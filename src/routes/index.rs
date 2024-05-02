use maud::{html, Markup};

use super::layout;

pub(crate) async fn render() -> Markup {
    layout(html!(
        h1 .bg-green-200 { "schedule" }
        a href="/new" { "Create new calendar" }
    ))
}
