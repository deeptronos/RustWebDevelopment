use axum::{extract::Query, response::Html, routing::get, Router};
use rand::{thread_rng, Rng}; // A random number generation library.
use serde::Deserialize; // A framework for serializing/deserializing Rust data structures generically.
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// `Deserialize` must be implemented to use with the `Query` extractor.
#[derive(Deserialize)]
struct RangeParameters {
    start: usize,
    end:   usize,
}

async fn handler(Query(range) : Query<RangeParameters>) -> Html<String> {
    // RNG based upon parse from query.
    let random_number = thread_rng().gen_range(range.start .. range.end);
    // Send HTML-format response.
    Html(format!("<title>{}</title> <h1>Random Number: {} </h1>", random_number, random_number))
}