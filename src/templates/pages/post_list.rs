use maud::{Markup, html};
use crate::templates::components::layout::layout;

pub fn post_list_page(posts: Vec<Post>) -> Markup {
    layout(
        "Posts",
        html! {
            section {
                h1 { "Posts" }
                
            }
        },
    )
}