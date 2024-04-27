use axum::{http::StatusCode, response::{Html, IntoResponse}, routing::get, Json, Router, http::{Uri}};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    
    let app = Router::new()
        .route("/read", get(read_handler))
        .fallback(fallback);

    // let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let addr = format!("127.0.0.1:3000");
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:3000"))
                                .await
                                .unwrap();
    println!("listening on {}", addr);
    axum::serve(listener, app)
    .await
    .unwrap();
    // axum::serve::bind(&addr)
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();

//     # DEMO:
//     let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port))
//     .await
//     .unwrap();
// axum::serve(listener, app).await.unwrap();
}

// async fn fallback(uri: Uri) -> (StatusCode, Html<&'static str>) {
async fn fallback(uri: Uri) -> (StatusCode,String) {
    // (StatusCode::NOT_FOUND, Html(include_str!("../res/static/404.html")))
    (StatusCode::NOT_FOUND, format!("No route for {}", uri))
}

async fn read_handler() -> impl IntoResponse{
    const MESSAGE: &str = "API services";

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
