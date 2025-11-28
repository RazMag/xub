use crate::templates::pages::write_page as write_template;
use anyhow::Result;
use axum::{extract::Form, http::StatusCode, response::IntoResponse};
use nanoid::nanoid;
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
    pub id: String,
}

pub async fn write_handler() -> impl IntoResponse {
    write_template()
}

pub async fn write_submit(_session: Session, Form(payload): Form<NewPost>) -> impl IntoResponse {
    // Placeholder: persist to storage here in the future
    let title = payload.title;
    let content = payload.content;
    let date = chrono::Utc::now();
    let frontmatter = Frontmatter {
        title: title.clone(),
        date: date.to_rfc3339(),
        id: nanoid!(),
    };
    let frontmatter_str = serde_saphyr::to_string(&frontmatter).unwrap();
    let post = format!("---\n{}---\n{}", frontmatter_str, content);
    save_post_to_file(&post, &frontmatter.id).await.unwrap();
    (StatusCode::CREATED, format!("Created post: {title}"))
}

async fn save_post_to_file(post: &str, id: &str) -> Result<()> {
    let mut file = fs::File::create(format!("./posts/{id}.md")).await?;
    file.write_all(post.as_bytes()).await?;
    Ok(())
}
