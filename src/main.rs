#![warn(clippy::all)]

mod api;
mod question;
mod questionbase;
mod web;
use api::*;
use question::*;
use questionbase::*;
use web::*;

// use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;

use std::io::Write;
use std::sync::Arc;

use clap::Parser;

use askama::Template;
use axum::{
    extract::{Path, State},
    http::{request::Parts, Method, StatusCode, Uri},
    response::{Html, IntoResponse, Response},
    routing::{delete, get, post, put},
    Json, Router,
};

use tower_http::{cors, services::ServeDir};

use tokio::{self, sync::RwLock};

use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};
extern crate serde_json;

use sqlx::{
    self,
    postgres::{PgConnection, PgPool, PgRow, Postgres},
    Pool, Row,
};
/// Represents the arguments passed to the program.
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// The IP address and port to serve the application on. Defaults to 0.0.0.0:3000.
    #[clap(short, long, default_value = "0.0.0.0:3000")]
    serve: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    startup(args.serve).await
}

/// Starts the web server application.
pub async fn startup(ip: String) {
    // Creates a new instance of `QuestionBase`. If an error occurs, it will be logged and the program will exit with a non-zero status code.
    let questionbase = QuestionBase::new().await.unwrap_or_else(|_e| {
        std::process::exit(1);
    });

    // Creates an `Arc` wrapper around the `RwLock<QuestionBase>`. This allows multiple threads to safely access the data within it concurrently while also allowing ownership transfer.
    let questionbase = Arc::new(RwLock::new(questionbase));

    let cors = cors::CorsLayer::new()
        .allow_methods([Method::GET])
        .allow_origin(cors::Any);

    // Initializes a new router for handling HTTP requests.
    // It includes routes for getting all questions, a specific question by its ID, adding a new question, deleting a question by its ID, and updating an existing question's details by its ID.
    let apis = Router::new()
        .route("/questions", get(questions))
        .route("/question", get(question))
        .route("/question/:id", get(get_handler))
        .route("/question/add", post(post_handler))
        .route("/question/:id", delete(delete_handler))
        .route("/question/:id", put(put_handler));

    // Initializes a new router for the web server application.
    // It includes routes for the home page and serving static assets from the "assets" directory.
    let app = Router::new()
        .route("/", get(handler_index))
        .route("/assets/templates/index.html", get(handler_index))
        .nest("/api/", apis)
        .nest_service("/assets", ServeDir::new("assets")) // Serves anything requested from /assets
        .fallback(fallback)
        .layer(cors)
        .with_state(questionbase);

    // Binds the server to the specified IP address and starts listening for incoming requests.
    let listener = tokio::net::TcpListener::bind(ip.clone()).await.unwrap();
    println!("listening on {}", ip);
    axum::serve(listener, app).await.unwrap();
}

/// Returns a html document representing a 404 page, and NOT_FOUND status code.
async fn fallback(uri: Uri) -> Response {
    println!("CTEST - fallback, uri: {:#?}", uri);
    (
        StatusCode::NOT_FOUND,
        Html(include_str!("../assets/static/404.html")),
    )
        .into_response()
}
