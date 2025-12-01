use crate::posts::Post;
use crate::templates::components::layout::layout;
use crate::templates::components::post::post_component;
use maud::{Markup, html};

pub fn post_list_page(posts: Vec<Post>) -> Markup {
    layout(
        "Posts",
        html! {
            section {
                h1 { "Posts" }
                @if posts.is_empty() {
                    p { "No posts available." }
                } @else {
                    @for post in posts {
                        (post_component(&post))
                    }
                }
            }
        },
    )
}
