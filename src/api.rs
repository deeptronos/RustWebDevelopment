// TODO ApiDoc!

use crate::*;
/// Returns a list of all questions in the database.
pub async fn questions(State(questionbase): State<Arc<RwLock<QuestionBase>>>) -> Response {
    let questions = questionbase.read().await.get_questions().await;
    (StatusCode::OK, Json(questions)).into_response()
}

/// Returns a random question from the database.
pub async fn question(State(questionbase): State<Arc<RwLock<QuestionBase>>>) -> Response {
    match questionbase.read().await.get_random().await {
        Ok(question) => question.into_response(),
        Err(e) => QuestionBaseError::response(StatusCode::NOT_FOUND, e),
    }
}

// GET request handler for retrieving a specific question by ID
/// Retrieves a single question from the database based on its unique ID.
pub async fn get_handler(
    State(questionbase): State<Arc<RwLock<QuestionBase>>>,
    Path(question_id): Path<String>,
) -> Response {
    // Attempt to retrieve the question from the database using its ID
    match questionbase.read().await.get(&question_id) {
        Ok(question) => question.into_response(),
        Err(e) => QuestionBaseError::response(StatusCode::NOT_FOUND, e),
    }
}

// POST request handler for adding a new question to the database
/// Adds a new question to the database based on the provided JSON payload.
pub async fn post_handler(
    State(questionbase): State<Arc<RwLock<QuestionBase>>>,
    Json(question): Json<Question>,
) -> Response {
    println!("CTEST _ DEBUG _ POST_HANDLER!!!");
    match questionbase.write().await.add(question).await {
        Ok(()) => StatusCode::CREATED.into_response(),
        Err(e) => QuestionBaseError::response(StatusCode::BAD_REQUEST, e),
    }
}

/// Deletes a question from the database based on its unique ID.
pub async fn delete_handler(
    State(questionbase): State<Arc<RwLock<QuestionBase>>>,
    Path(question_id): Path<String>,
) -> Response {
    match questionbase.write().await.delete(&question_id) {
        Ok(()) => StatusCode::OK.into_response(),
        Err(e) => QuestionBaseError::response(StatusCode::BAD_REQUEST, e),
    }
}

/// Updates a question in the database based on its unique ID and JSON payload.
pub async fn put_handler(
    State(questionbase): State<Arc<RwLock<QuestionBase>>>,
    Path(question_id): Path<String>,
    Json(question): Json<Question>,
) -> Response {
    match questionbase.write().await.update(&question_id, question) {
        Ok(_) => StatusCode::OK.into_response(),
        Err(QuestionBaseErr::QuestionUnprocessable(e)) => QuestionBaseError::response(
            StatusCode::UNPROCESSABLE_ENTITY,
            QuestionBaseErr::QuestionUnprocessable(e),
        ),
        Err(QuestionBaseErr::NoQuestion) => {
            QuestionBaseError::response(StatusCode::NOT_FOUND, QuestionBaseErr::NoQuestion)
        }
        Err(e) => QuestionBaseError::response(StatusCode::BAD_REQUEST, e),
    }
}
