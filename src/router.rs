use axum::{
    Router,
    routing::{get, post},
    middleware::{from_fn, Next},
    extract::{Path, Request, FromRequestParts},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use tower_sessions::Session;
use serde::Deserialize;

pub fn build_router() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/posts", get(posts))
        .route("/post/{id}", get(show_post))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/secret", get(secret).route_layer(from_fn(require_auth)))
}

async fn root() -> &'static str {
    "Hello from xub!"
}

async fn posts() -> &'static str {
    "Posts index"
}

async fn show_post(Path(id): Path<String>) -> String {
    format!("Post {id}")
}

#[derive(Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

async fn login(session: Session, Json(payload): Json<LoginPayload>) -> impl IntoResponse {
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
    (StatusCode::OK, "Logged out")
}

async fn secret() -> &'static str {
    "Top secret content"
}

async fn require_auth(req: Request, next: Next) -> Result<Response, StatusCode> {
    let (mut parts, body) = req.into_parts();

    // Extract Session from request parts (requires the tower-sessions layer).
    let session = match Session::from_request_parts(&mut parts, &()).await {
        Ok(s) => s,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };

    let is_logged_in = session.get::<bool>("logged_in").await.ok().flatten().unwrap_or(false);
    if is_logged_in {
        let req = Request::from_parts(parts, body);
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}
