use crate::posts::Post;
use crate::templates::components::layout::layout;
use maud::{Markup, PreEscaped, html};

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
                        article {
                            h2 { (post.title) }
                            (PreEscaped(post.body_html))
                        }
                    }
                }
            }
        },
    )
}
