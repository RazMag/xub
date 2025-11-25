use maud::{Markup, html};

use crate::templates::components::layout::layout;


pub fn logout_page() -> Markup {
    layout(
        "Logged Out",
        html! {
            section {
                h1 { "Logged Out" }
                p { "You have ended the session." }
                a href="/" { "Go back home" }
            }
        },
    )
}