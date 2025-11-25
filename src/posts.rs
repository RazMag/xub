use std::path::PathBuf;

use pulldown_cmark::{html, Options, Parser};
use tokio::fs;

const POSTS_DIR: &str = "posts";

#[derive(Debug, Clone)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub body_html: String,
}

#[derive(Debug)]
pub enum PostError {
    NotFound(String),
    Io(std::io::Error),
}

impl From<std::io::Error> for PostError {
    fn from(err: std::io::Error) -> Self {
        PostError::Io(err)
    }
}

pub async fn load_post(id: &str) -> Result<Post, PostError> {
    let path = post_path(id);
    let markdown = fs::read_to_string(&path)
        .await
        .map_err(|err| match err.kind() {
            std::io::ErrorKind::NotFound => PostError::NotFound(id.to_string()),
            _ => PostError::Io(err),
        })?;

    Ok(parse_post(id.to_string(), &markdown))
}

pub async fn load_all_posts() -> Result<Vec<Post>, PostError> {
    let mut posts = Vec::new();
    let mut entries = match fs::read_dir(POSTS_DIR).await {
        Ok(entries) => entries,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(posts),
        Err(err) => return Err(PostError::Io(err)),
    };

    while let Some(entry) = entries.next_entry().await.map_err(PostError::Io)? {
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("md") {
            continue;
        }

        let file_name = entry.file_name();
        let id = file_name
            .to_string_lossy()
            .trim_end_matches(".md")
            .to_string();

        let markdown = match fs::read_to_string(&path).await {
            Ok(data) => data,
            Err(err) => {
                eprintln!("failed to read post {id}: {err}");
                continue;
            }
        };

        posts.push(parse_post(id, &markdown));
    }

    posts.sort_by(|a, b| b.id.cmp(&a.id));
    Ok(posts)
}

fn parse_post(id: String, markdown: &str) -> Post {
    let title = extract_title(markdown).unwrap_or_else(|| id.clone());
    let body_html = render_markdown(markdown);

    Post {
        id,
        title,
        body_html,
    }
}

fn render_markdown(markdown: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);
    let parser = Parser::new_ext(markdown, options);
    let mut output = String::new();
    html::push_html(&mut output, parser);
    output
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

fn post_path(id: &str) -> PathBuf {
    let mut path = PathBuf::from(POSTS_DIR);
    path.push(format!("{id}.md"));
    path
}
