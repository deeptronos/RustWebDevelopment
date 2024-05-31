// TODO: Find out why none of my migrations/ tables are appearing, and add some example data
//          (in the style of Bart's example)

// TODO ApiDoc!

use crate::*;

pub async fn questions(State(questionbase): State<Arc<RwLock<QuestionBase>>>) -> Response {
    let questions = questionbase.read().await.get_questions().await;
    (StatusCode::OK, Json(questions)).into_response()
}

pub async fn question(State(questionbase): State<Arc<RwLock<QuestionBase>>>) -> Response {
    match questionbase.read().await.get_random().await {
        Ok(question) => question.into_response(),
        Err(e) => QuestionBaseError::response(StatusCode::NOT_FOUND, e),
    }
}

// pub async fn get_question(
//     State(questionbase) : State<Arc<RwLock<QuestionBase>>>,
//     Path(question_id) : Path<String>,) ->
//     Response {
//     match questionbase.read().await.get(&question_id) {
//         Ok(question) => question.into_response(),
//         Err(e) => QuestionBaseError::response(StatusCode::NOT_FOUND, e),
//     }
// }

// pub async fn get_question(
pub async fn get_handler(
    State(questionbase): State<Arc<RwLock<QuestionBase>>>,
    Path(question_id): Path<String>,
) -> Response {
    match questionbase.read().await.get(&question_id) {
        Ok(question) => question.into_response(),
        Err(e) => QuestionBaseError::response(StatusCode::NOT_FOUND, e),
    }
}

// pub async fn post_question(
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

// pub async fn delete_question(
pub async fn delete_handler(
    State(questionbase): State<Arc<RwLock<QuestionBase>>>,
    Path(question_id): Path<String>,
) -> Response {
    match questionbase.write().await.delete(&question_id) {
        Ok(()) => StatusCode::OK.into_response(),
        Err(e) => QuestionBaseError::response(StatusCode::BAD_REQUEST, e),
    }
}

// pub async fn update_question(
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
