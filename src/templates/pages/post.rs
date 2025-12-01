use crate::posts::Post;
use crate::templates::components::layout::layout;
use crate::templates::components::post::post_component;
use maud::{Markup, html};

pub fn post_page(post: Post) -> Markup {
    layout(
        &post.title,
        html! {
            (post_component(&post))
        },
    )
}