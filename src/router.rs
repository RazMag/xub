use crate::posts::{self, load_post};
use crate::templates::pages;
use crate::write::{write_handler, write_submit};
use axum::extract::Path;
use axum::{
    Router,
    extract::{Form, FromRequestParts, Request},
    http::StatusCode,
    middleware::{Next, from_fn},
    response::{IntoResponse, Redirect, Response},
    routing::get,
};
use serde::Deserialize;
use tower_sessions::Session;

pub async fn build_router() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/posts", get(posts_index))
        .route("/post/{id}", get(post_handler))
        .route("/login", get(login_handler).post(login_submit))
        .route("/logout", get(logout_handler))
        .route(
            "/write",
            get(write_handler)
                .post(write_submit)
                .route_layer(from_fn(require_auth)),
        )
}

async fn root() -> impl IntoResponse {
    Redirect::to("/posts")
}

async fn post_handler(Path(id): Path<String>) -> impl IntoResponse {
    pages::post_page(load_post(&id).await.unwrap())
}

async fn posts_index() -> Response {
    match posts::load_all_posts().await {
        Ok(mut posts) => {
            posts.sort_by(|a, b| b.created.cmp(&a.created)); // Newest first
            pages::post_list_page(posts).into_response()
        }
        Err(err) => {
            eprintln!("failed to load posts: {err:?}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

#[derive(Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

async fn login_submit(session: Session, Form(payload): Form<LoginPayload>) -> impl IntoResponse {
    let expected_user = std::env::var("LOGIN_USER").unwrap_or_else(|_| "admin".to_string());
    let expected_pass = std::env::var("LOGIN_PASS").unwrap_or_else(|_| "password".to_string());

    if payload.username == expected_user && payload.password == expected_pass {
        // Store user identity in the session
        let _ = session.insert("user", &payload.username).await;
        let _ = session.insert("logged_in", true).await;
        StatusCode::OK.into_response()
    } else {
        (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response()
    }
}

async fn logout_handler(session: Session) -> impl IntoResponse {
    let _ = session.flush().await;
    pages::logout_page()
}

async fn login_handler() -> impl IntoResponse {
    pages::login_page()
}

async fn require_auth(req: Request, next: Next) -> Result<Response, StatusCode> {
    let (mut parts, body) = req.into_parts();

    // Extract Session from request parts (requires the tower-sessions layer).
    let session = match Session::from_request_parts(&mut parts, &()).await {
        Ok(s) => s,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };

    let is_logged_in = session
        .get::<bool>("logged_in")
        .await
        .ok()
        .flatten()
        .unwrap_or(false);
    if is_logged_in {
        let req = Request::from_parts(parts, body);
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}
