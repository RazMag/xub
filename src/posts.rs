use axum::response::Html;
use pulldown_cmark::{html, Options, Parser};
use tokio::fs;

#[derive(Debug)]
struct RenderedPost {
	id: String,
	title: String,
	html: String,
}

pub async fn posts_page() -> Html<String> {
	// Read all markdown files from ./posts, parse, sort desc by filename
	let mut posts: Vec<RenderedPost> = Vec::new();

	let mut dir = match fs::read_dir("posts").await {
		Ok(d) => d,
		Err(_) => return Html(simple_page("<p>No posts directory found.</p>")),
	};

	while let Ok(Some(entry)) = dir.next_entry().await {
		let file_name = entry.file_name();
		let file_name = file_name.to_string_lossy();
		if !file_name.ends_with(".md") {
			continue;
		}

		let id = file_name.trim_end_matches(".md").to_string();
		let path = entry.path();
		let md = match fs::read_to_string(&path).await {
			Ok(s) => s,
			Err(_) => continue,
		};

		let mut _options = Options::empty();
		_options.insert(Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);
		let parser = Parser::new_ext(&md, _options);
		let mut html_output = String::new();
		html::push_html(&mut html_output, parser);

		let title = extract_title(&md).unwrap_or_else(|| id.clone());

		posts.push(RenderedPost { id, title, html: html_output });
	}

	// Sort newest first by id (filenames are timestamps)
	posts.sort_by(|a, b| b.id.cmp(&a.id));

	// Render minimal HTML without styling
	let mut body = String::new();
	body.push_str("<h1>Posts</h1>");
	for p in posts {
		body.push_str("<article>");
		// optional anchor with id
		body.push_str(&format!("<a id=\"{}\"></a>", p.id));
		body.push_str(&format!("<h2>{}</h2>", html_escape::encode_text(&p.title)));
		body.push_str(&p.html);
		body.push_str("</article>\n<hr />\n");
	}

	Html(simple_page(&body))
}

fn simple_page(body: &str) -> String {
	format!(
		r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <title>Posts</title>
</head>
<body>
{}
</body>
</html>"#,
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
		let t = line.trim_end();
		if t == "---" {
			break;
		}
		if let Some(rest) = t.strip_prefix("title:") {
			let value = rest.trim();
			let value = value.trim_matches('"').trim_matches('\'');
			if !value.is_empty() {
				title = Some(value.to_string());
			}
		}
	}

	title
}


