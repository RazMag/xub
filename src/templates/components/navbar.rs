use maud::{Markup, html};

pub fn navbar() -> Markup {
    html! {
        nav .navbar {
            a href="/" { "Home" }
            a href="/posts" { "Posts" }
            a href="/write" { "Write" }
            a href="/login" { "Login" }
            a href="/about" { "About" }
        }
    }
}
