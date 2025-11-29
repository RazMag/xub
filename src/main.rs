use std::net::SocketAddr;

use anyhow::Result;
use axum::Router;
use tokio::net::TcpListener;
use tower_sessions::{MemoryStore, SessionManagerLayer};


mod posts;
mod router;
mod templates;
mod write;

#[tokio::main]
async fn main() -> Result<()> {
    // Create the app router with routes defined in router module.
    let app: Router = router::build_router().await;

    // Session layer (MemoryStore for dev). with_secure(false) for HTTP during dev.
    let store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(store).with_secure(false);
    let app = app.layer(session_layer);

    // Read PORT from env or default to 3000
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(3000);

    // Bind and serve
    let addr: SocketAddr = ([127, 0, 0, 1], port).into(); //TODO make configurable
    let listener = TcpListener::bind(addr).await?;
    println!("listening on http://{}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
