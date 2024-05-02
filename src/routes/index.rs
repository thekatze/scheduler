use maud::{html, Markup};

use crate::routes::{header, submit_button};

use super::layout;

pub(crate) async fn render() -> Markup {
    layout(html!(
        (header("schedule"))
        p { "Easily create a shared calendar for events." }
        form .flex .gap-4 action="/new" method="POST" {
            input .px-4 .py-2 .bg-indigo-100 .text-indigo-900 .rounded type="text" name="name" placeholder="Calendar name" required;
            (submit_button("Create"))
        }
    ))
}
