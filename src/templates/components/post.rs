use crate::posts::Post;
use maud::{Markup, PreEscaped, html};

pub fn post_component(post: &Post) -> Markup {
    html! {
        article {
            h2 { (post.title) }
            (PreEscaped(&post.body_html))
        }
    }
}
