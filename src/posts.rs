use std::{cmp::Ordering, path::PathBuf};

use chrono::{DateTime, Utc};
use pulldown_cmark::{Options, Parser, html};
use tokio::fs;

const POSTS_DIR: &str = "./posts";

#[derive(Debug, Clone)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub body_html: String,
    pub created: Option<DateTime<Utc>>,
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

    posts.sort_by(|a, b| match (&a.created, &b.created) {
        (Some(a_date), Some(b_date)) => b_date.cmp(a_date),
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => b.id.cmp(&a.id),
    });
    Ok(posts)
}

fn parse_post(id: String, markdown: &str) -> Post {
    let title = extract_title(markdown).unwrap_or_else(|| id.clone());
    let body_html = render_markdown(markdown);
    let created = extract_created(markdown);

    Post {
        id,
        title,
        body_html,
        created,
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
    extract_frontmatter_value(md, "title")
}

fn extract_created(md: &str) -> Option<DateTime<Utc>> {
    let value = extract_frontmatter_value(md, "created")?;
    chrono::DateTime::parse_from_rfc3339(&value)
        .map(|dt| dt.with_timezone(&Utc))
        .ok()
}

fn extract_frontmatter_value(md: &str, key: &str) -> Option<String> {
    let mut lines = md.lines();
    let first = lines.next()?;
    if first.trim() != "---" {
        return None;
    }

    let search_key = format!("{key}:");
    for line in lines {
        let trimmed = line.trim_end();
        if trimmed == "---" {
            break;
        }
        if let Some(rest) = trimmed.strip_prefix(&search_key) {
            let value = rest.trim();
            let value = value.trim_matches('"').trim_matches('\'');
            if !value.is_empty() {
                return Some(value.to_string());
            }
        }
    }

    None
}

fn post_path(id: &str) -> PathBuf {
    let mut path = PathBuf::from(POSTS_DIR);
    path.push(format!("{id}.md"));
    path
}
