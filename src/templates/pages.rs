use maud::{Markup, html};

use crate::templates::layout::layout;

pub fn home_page() -> Markup {
    layout(
        "Welcome",
        html! {
            section {
                h1 { "Hello from xub!" }
                p { "A place to write" }
            }
        },
    )
}

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

pub fn secret_page() -> Markup {
    layout(
        "Secret",
        html! {
            section {
                h1 { "Top Secret" }
                p { "Authorized eyes only." }
            }
        },
    )
} //TODO remove

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
