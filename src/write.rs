use anyhow::Result;
use axum::{
    extract::Form,
    http::StatusCode,
    response::{Html, IntoResponse},
};
use serde::{Deserialize, Serialize};
use tokio::{fs, io::AsyncWriteExt};
use tower_sessions::Session;

#[derive(Deserialize)]
pub struct NewPost {
    pub title: String,
    pub content: String,
}

#[derive(Serialize)]
pub struct Frontmatter {
    pub title: String,
    pub date: String,
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
    let content = payload.content;
    let date = chrono::Utc::now();
    let frontmatter = Frontmatter {
        title: title.clone(),
        date: date.to_rfc3339(),
    };
    let frontmatter = serde_yaml::to_string(&frontmatter).unwrap();
    let post = format!("---\n{}---\n{}", frontmatter, content);
    save_post_to_file(&post).await.unwrap();
    // For now, just acknowledge creation
    (StatusCode::CREATED, format!("Created post: {title}"))
}

async fn save_post_to_file(post: &str) -> Result<()> {
    let filename = format!("posts/{}.md", chrono::Utc::now().format("%Y%m%d%H%M%S"));
    let mut file = fs::File::create(&filename).await?;
    file.write_all(post.as_bytes()).await?;
    Ok(())
}
