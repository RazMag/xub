use crate::post::post_page;
use crate::post_list::posts_page;
use crate::templates::pages::{
    home_page as home_template, login_page as login_template, logout_page as logout_template,
    secret_page as secret_template,
};
use crate::write::{write_page, write_submit};
use axum::{
    Router,
    extract::{Form, FromRequestParts, Request},
    http::StatusCode,
    middleware::{Next, from_fn},
    response::{IntoResponse, Response},
    routing::get,
};
use serde::Deserialize;
use tower_sessions::Session;

pub fn build_router() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/posts", get(posts_page))
        .route("/post/{id}", get(post_page))
        .route("/login", get(login_page).post(login_submit))
        .route("/logout", get(logout))
        .route(
            "/write",
            get(write_page).post(write_submit), // .route_layer(from_fn(require_auth)), //TODO add back auth
        )
        .route("/secret", get(secret).route_layer(from_fn(require_auth)))
}

async fn root() -> impl IntoResponse {
    home_template()
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

async fn logout(session: Session) -> impl IntoResponse {
    let _ = session.flush().await;
    logout_template()
}

async fn login_page() -> impl IntoResponse {
    login_template()
}

async fn secret() -> impl IntoResponse {
    secret_template()
} //TODO remove

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
