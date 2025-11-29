use crate::posts::Post;
use crate::templates::components::layout::layout;
use maud::{Markup, PreEscaped, html};

pub fn post_page(post: Post) -> Markup {
    layout(
        &post.title,
        html! {
            article {
                h1 { (post.title) }
                (PreEscaped(post.body_html))
            }
        },
    )
}