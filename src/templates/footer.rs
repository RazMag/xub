use maud::{Markup, html};

pub fn footer() -> Markup {
    html! {
        footer {
            p {
                "made with ❤️ by "
                a href="https://github.com/razmag" { "razmag" }
            }
        }
    }
}
