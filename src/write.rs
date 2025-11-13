use axum::{
    extract::Form,
    http::StatusCode,
    response::{Html, IntoResponse},
};
use serde::Deserialize;
use tower_sessions::Session;

#[derive(Deserialize)]
pub struct NewPost {
    pub title: String,
    pub content: String,
}

pub async fn write_page() -> Html<&'static str> {
    Html(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <title>Write a Post</title>
</head>
<body>
  <h2>Write a new post</h2>
  <form method="post" action="/write">
    <label for="title">Title</label><br />
    <input id="title" name="title" type="text" required /><br />
    <label for="content">Content</label><br />
    <textarea id="content" name="content" rows="8" cols="40" required></textarea><br />
    <button type="submit">Publish</button>
  </form>
</body>
</html>"#,
    )
}

pub async fn write_submit(_session: Session, Form(payload): Form<NewPost>) -> impl IntoResponse {
    // Placeholder: persist to storage here in the future
    let title = payload.title;
    let _content = payload.content;

    // For now, just acknowledge creation
    (StatusCode::CREATED, format!("Created post: {title}"))
}
