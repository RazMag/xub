use maud::{Markup, html};

use crate::templates::components::{footer::footer, navbar::navbar};

pub fn layout(title: &str, content: Markup) -> Markup {
    html! {
        (maud::DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { (title) }
            }
            body {
                (navbar())
                main {
                    (content)
                }
                (footer())
            }
        }
    }
}
