use maud::{html, Markup};

pub fn navbar() -> Markup {
    html! {
        nav .navbar {
            a href="/" { "Home" }
            a href="/posts" { "Posts" }
            a href="/about" { "About" }
        }
    }
}
