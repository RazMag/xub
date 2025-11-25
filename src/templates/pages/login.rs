use maud::{Markup, html};

use crate::templates::components::layout::layout;


pub fn login_page() -> Markup {
    layout(
        "Login",
        html! {
            section {
                h1 { "Login" }
                form method="post" action="/login" {
                    label for="username" { "Username" }
                    input id="username" name="username" type="text" required;
                    label for="password" { "Password" }
                    input id="password" name="password" type="password" required;
                    button type="submit" { "Sign In" }
                }
            }
        },
    )
}