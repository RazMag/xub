use maud::{Markup, html};

use crate::templates::components::layout::layout;

pub fn write_page() -> Markup {
    layout(
        "Write a Post",
        html! {
            section {
                h1 { "Write a new post" }
                form method="post" action="/write" {
                    label for="title" { "Title" }
                    input id="title" name="title" type="text" required;
                    label for="content" { "Content" }
                    textarea id="content" name="content" required {};
                    button type="submit" { "Publish" }
                }
            }
        },
    )
}
