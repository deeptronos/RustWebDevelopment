use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Route all requests on "/" endpoint to anonymous handler.
    //
    // A handler is an async function which returns something that implements
    // `axum::response::IntoResponse`.

    // A closure or a function can be used as handler.
    let app = Router::new().route("/", get(handler));


    let addr = SocketAddr::from(([127, 0, 0, 1], 3000)); // Address to bind the server to.

    axum::Server::bind(&addr) 
        .serve(app.into_make_service()) // Convert app router into a MakeService.
        .await
        .unwrap();

}

async fn handler() -> &'static str {
    "Hello, world"
}