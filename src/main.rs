mod api;
mod question;
mod questionbase;
mod web;
use api::*;
use question::*;
use questionbase::*;
use web::*;

use std::collections::HashMap;
use std::fs::File;
use std::io::{ErrorKind, Seek, Write};
use std::sync::Arc;

use askama::Template;
use axum::{
    extract::{Path, State},
    http::{StatusCode, Uri},
    response::{Html, IntoResponse, Response},
    routing::{delete, get, post, put},
    Json, Router,
};

use tower_http::services::ServeDir;

use tokio::{self, sync::RwLock};

use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};
extern crate serde_json;

#[tokio::main]
async fn main() {
    let questionbase = QuestionBase::new("assets/questionbase.json").unwrap_or_else(|_e| {
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
        .nest_service("/assets", ServeDir::new("assets")) // Serve anything requested from /assets
        .fallback(fallback)
        .with_state(questionbase);

    let addr = "127.0.0.1:3000".to_string(); // TODO useless
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}

/// Returns a html document representing a 404 page, and NOT_FOUND status code.
async fn fallback(uri: Uri) -> Response {
    println!("uri: {:#?}", uri);
    (
        StatusCode::NOT_FOUND,
        Html(include_str!("../res/static/404.html")),
    )
        .into_response()
}
