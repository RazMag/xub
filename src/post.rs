use std::path::PathBuf;

use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use html_escape::encode_text;
use pulldown_cmark::{Options, Parser, html};
use tokio::fs;

enum PostError {
    NotFound,
    Io(std::io::Error),
}

pub async fn post_page(Path(id): Path<String>) -> Response {
    match render_post(&id).await {
        Ok(html) => Html(html).into_response(),
        Err(PostError::NotFound) => (
            StatusCode::NOT_FOUND,
            Html(simple_page(
                "Post Not Found",
                &format!(
                    "<h1>Post not found</h1><p>No post available for id <code>{}</code>.</p>",
                    encode_text(&id)
                ),
            )),
        )
            .into_response(),
        Err(PostError::Io(err)) => {
            eprintln!("failed to render post {id}: {err}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Html(simple_page(
                    "Post Error",
                    "<h1>Something went wrong</h1><p>Unable to load this post.</p>",
                )),
            )
                .into_response()
        }
    }
}

async fn render_post(id: &str) -> Result<String, PostError> {
    let mut path = PathBuf::from("posts");
    path.push(format!("{id}.md"));

    let markdown = fs::read_to_string(&path)
        .await
        .map_err(|err| match err.kind() {
            std::io::ErrorKind::NotFound => PostError::NotFound,
            _ => PostError::Io(err),
        })?;

    let mut options = Options::empty();
    options.insert(Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);
    let parser = Parser::new_ext(&markdown, options);
    let mut rendered = String::new();
    html::push_html(&mut rendered, parser);

    let title = extract_title(&markdown).unwrap_or_else(|| id.to_string());
    let mut body = String::new();
    body.push_str(&format!("<article><h1>{}</h1>", encode_text(&title)));
    body.push_str(&rendered);
    body.push_str("</article>");

    Ok(simple_page(&title, &body))
}

fn simple_page(title: &str, body: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <title>{}</title>
</head>
<body>
{}
</body>
</html>"#,
        encode_text(title),
        body
    )
}

fn extract_title(md: &str) -> Option<String> {
    let mut lines = md.lines();
    let first = lines.next()?;
    if first.trim() != "---" {
        return None;
    }

    let mut title: Option<String> = None;
    for line in lines {
        let trimmed = line.trim_end();
        if trimmed == "---" {
            break;
        }
        if let Some(rest) = trimmed.strip_prefix("title:") {
            let value = rest.trim();
            let value = value.trim_matches('"').trim_matches('\'');
            if !value.is_empty() {
                title = Some(value.to_string());
            }
        }
    }

    title
}
