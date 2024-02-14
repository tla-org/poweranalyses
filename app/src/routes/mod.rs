use axum::{
    routing::{get, post},
    Router,
};

use power::Tail;
use power::TestKind;

pub mod backend;

// TODO:
pub fn routes() -> Router {
    Router::new()
        .route("/n", get(backend::backend))
        .route("/n", post(backend::backend))
}
