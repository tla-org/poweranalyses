use std::path::Path;

use anyhow::{Context, Result};
use axum::{serve, Router};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[cfg(feature = "livereload")]
use axum::http::Request;
#[cfg(feature = "livereload")]
use notify::{RecursiveMode::Recursive, Watcher};
#[cfg(feature = "livereload")]
use tower_livereload::LiveReloadLayer;
#[cfg(feature = "livereload")]
fn not_htmx_predicate<T>(req: &Request<T>) -> bool {
    !req.headers().contains_key("hx-request")
}

async fn serve_app() -> Result<()> {
    info!("Initializing router...");

    let assets_path = Path::new("app/assets");
    let app = Router::new().nest_service("/", ServeDir::new(assets_path));

    #[cfg(feature = "livereload")]
    let (app, port) = {
        info!("LiveReload enabled.");
        let livereload = LiveReloadLayer::new().request_predicate(not_htmx_predicate);
        let reloader = livereload.reloader();
        let mut watcher = notify::recommended_watcher(move |_| reloader.reload())?;
        watcher.watch(assets_path, Recursive)?;

        (app.layer(livereload), 3000) // Use port 3000 for livereload
    };

    #[cfg(not(feature = "livereload"))]
    let port = 443; // Use port 443 without livereload

    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    serve(listener, app)
        .await
        .context("Error while starting server")?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // Set up logging
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    serve_app().await?;

    Ok(())
}
