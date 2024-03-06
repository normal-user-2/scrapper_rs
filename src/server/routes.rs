use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;

fn init_tracing() {
    let subscriber = tracing_subscriber::fmt()
        .with_file(true)
        .with_line_number(true)
        .without_time()
        .finish();
    let _ = tracing::subscriber::set_global_default(subscriber);
}

pub fn set_routes() -> Router {
    init_tracing();
    Router::new()
        .route("/", get(root))
        .route("/sync", post(sync))
}

async fn root() -> &'static str {
    "ok"
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct SyncRequest {
    password: String,
}

async fn sync(Json(req): Json<SyncRequest>) -> (StatusCode, &'static str) {
    let pass = req.password;

    println!("password: {}", pass);

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, "Started")
}
