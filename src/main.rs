mod api;
mod question;
mod questionbase;
mod web;
use axum::handler;
use api::*;
use question::*;
use questionbase::*;
use web::*;


use std::fs::File;
use std::io::{ErrorKind, Seek, Write};
use std::net::SocketAddr;
use std::sync::Arc;
use std::collections::{HashMap, HashSet};


use askama::Template;
use axum::{
    http::{StatusCode, Uri}, 
    response::{Html, IntoResponse, Response}, 
    extract::{Path, State},
    routing::{post, get, delete, put}, 
    serve::Serve, 
    Json, Router,};

use tower_http::services::ServeDir;


use tokio::{self, sync::RwLock};

use serde::{Serialize, Serializer, ser::SerializeStruct, Deserialize};
extern crate serde_json;

#[tokio::main]
async fn main() {
    let questionbase = QuestionBase::new("assets/questionbase.json")
                    .unwrap_or_else(|e|{
                        std::process::exit(1);
                    });
    let questionbase = Arc::new(RwLock::new(questionbase));

    let apis = Router::new()
                                .route("/questions", get(questions))
                                .route("/question", get(question))
                                .route("/question/:id", get(get_handler))
                                .route("/question/add", post(post_handler))
                                .route("/question/:id", delete(delete_handler))
                                .route("/question/:id", put(put_handler));

    let app = Router::new()
        .route("/", get(handler_index))                
        .route("/index.html", get(handler_index))
        .nest("/api/", apis)
        // .route("/create", post(create_handler))           // POST
        // .route("/update", put(update_handler)) // PUT
        // .route("/delete", delete(delete_handler))         // DELETE
        .nest_service("/assets", ServeDir::new("assets")) // Serve anything requested from /assets
        .fallback(fallback)
        .with_state(questionbase);


    let addr = format!("127.0.0.1:3000"); // TODO useless
    let listener = tokio::net::TcpListener::bind(&addr)
                                .await
                                .unwrap();
    println!("listening on {}", addr);
    axum::serve(listener, app)
    .await
    .unwrap();

}

/// Returns a 404 page and NOT_FOUND status code.
// async fn fallback(uri: Uri) -> (StatusCode, Html<&'static str>) {
async fn fallback(uri: Uri) -> Response {
    println!("uri: {:#?}",uri );
    (StatusCode::NOT_FOUND, Html(include_str!("../res/static/404.html"))).into_response()
    // (StatusCode::NOT_FOUND, format!("No route for {}", uri))
}


// /// Create a new Question! 
// /// Corresponds to the `POST` method.
// async fn create_handler() -> impl IntoResponse {
//     const MESSAGE: &str = "API service - CREATE";

//     let json_response = serde_json::json!({
//         "status" : "OK",
//         "message" : MESSAGE
//     });
//     Json(json_response) 
// }

// /// Read a random Question!
// /// Corresponds to the `READ` method.
// async fn read_handler() -> impl IntoResponse {
//     const MESSAGE: &str = "API service - READ";

//     let json_response = serde_json::json!({
//         "status" : "OK",
//         "message" : MESSAGE
//     });
//     Json(json_response)
// }

// /// Update a random Question!
// /// Corresponds to the `PUT` method.
// async fn update_handler() -> impl IntoResponse {
//     const MESSAGE: &str = "API service - UPDATE";

//     let json_response = serde_json::json!({
//         "status" : "OK",
//         "message" : MESSAGE
//     });
//     Json(json_response)
// }

// /// Delete a random Post 
// async fn delete_handler() -> impl IntoResponse {
//     const MESSAGE: &str = "API service - DELETE";

//     let json_response = serde_json::json!({
//         "status" : "OK",
//         "message" : MESSAGE
//     });
//     Json(json_response)
// }


