use super::*;

async fn backend() -> String {
    let test = TestKind::OneSampleTTest;
    let n = test.n(Tail::OneSided, 0.05, 0.8, 0.2);
    format!("Hello from backend! n: {n}")
}

pub fn routes() -> Router {
    Router::new().route("/n", get(backend))
}
