use axum::{http::{StatusCode, Uri}, response::{Html, IntoResponse}, routing::{delete, get}, serve::Serve, Json, Router};
use tower_http::services::ServeDir;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    
    let app = Router::new()
        .route("/create", get(create_handler))
        .route("/read", get(read_handler))
        .route("/update", get(update_handler))
        .route("/delete", get(delete_handler))
        .nest_service("/assets", ServeDir::new("assets")) // Serve anything requested from /assets
        .fallback(fallback);


    let addr = format!("127.0.0.1:3000"); // TODO useless
    let listener = tokio::net::TcpListener::bind(&addr)
                                .await
                                .unwrap();
    println!("listening on {}", addr);
    axum::serve(listener, app)
    .await
    .unwrap();

}

// async fn fallback(uri: Uri) -> (StatusCode, ) {
async fn fallback(uri: Uri) -> (StatusCode, Html<&'static str>) {
    println!("uri: {:#?}",uri );
    (StatusCode::NOT_FOUND, Html(include_str!("../res/static/404.html")))
    // (StatusCode::NOT_FOUND, format!("No route for {}", uri))
}

// CRUD:
async fn create_handler() -> impl IntoResponse {
    const MESSAGE: &str = "API service - CREATE";

    let json_response = serde_json::json!({
        "status" : "OK",
        "message" : MESSAGE
    });
    Json(json_response) 
}

async fn read_handler() -> impl IntoResponse {
    const MESSAGE: &str = "API service - READ";

    let json_response = serde_json::json!({
        "status" : "OK",
        "message" : MESSAGE
    });
    Json(json_response)
}

async fn update_handler() -> impl IntoResponse {
    const MESSAGE: &str = "API service - UPDATE";

    let json_response = serde_json::json!({
        "status" : "OK",
        "message" : MESSAGE
    });
    Json(json_response)
}

async fn delete_handler() -> impl IntoResponse {
    const MESSAGE: &str = "API service - DELETE";

    let json_response = serde_json::json!({
        "status" : "OK",
        "message" : MESSAGE
    });
    Json(json_response)
}


// async fn read_handler() -> Html<&'static str> {
//     // `std::include_str` macro can be used to include an utf-8 file as `&'static str` in compile
//     // time. This method is relative to current `main.rs` file.
//     Html(include_str!("../index.html"))
// }
