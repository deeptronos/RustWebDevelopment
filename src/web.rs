use crate::*;

/// Represents the template for the index page.
///
/// This struct is used to render the index page with the appropriate data.
///
/// # Fields
///
/// * `question` - An optional reference to a question.
/// * `tags` - An optional string representing tags.
/// * `stylesheet` - A static string representing the path to the stylesheet.
/// * `error` - An optional string representing an error message.
#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
    question: Option<&'a Question>,
    tags: Option<String>,
    stylesheet: &'static str,
    error: Option<String>,
}

impl<'a> IndexTemplate<'a> {
    /// Creates a new `IndexTemplate` with a question and no error.
    ///
    /// # Arguments
    ///
    /// * `question` - A reference to a question.
    ///
    /// # Returns
    ///
    /// A new `IndexTemplate` with the provided question and the default stylesheet.
    fn question(question: &'a Question) -> Self {
        Self {
            question: Some(question),
            tags: None,
            stylesheet: "/question.css",
            error: None,
        }
    }

    /// Creates a new `IndexTemplate` with an error and no question.
    ///
    /// # Arguments
    ///
    /// * `error` - A string representing an error message.
    ///
    /// # Returns
    ///
    /// A new `IndexTemplate` with the provided error message and the default stylesheet.
    fn error(error: String) -> Self {
        Self {
            question: None,
            tags: None,
            stylesheet: "/question.css",
            error: Some(error),
        }
    }
}
/// Handles the index page request.
///
/// This function retrieves a random question from the question base and returns an `IndexTemplate`
/// with the question or an error message, if any.
///
/// # Arguments
///
/// * `question_base` - A state reference to a question base.
///
/// # Returns
///
/// A `Response` containing an `IndexTemplate` with the question or an error message.
pub async fn handler_index(State(question_base): State<Arc<RwLock<QuestionBase>>>) -> Response {
    // Attempt to retrieve a random question from the question base
    match question_base.read().await.get_random().await {
        // If successful, return an `IndexTemplate` with the question and a status code of OK
        Ok(question) => (StatusCode::OK, IndexTemplate::question(&question)).into_response(),
        // If an error occurs, return an `IndexTemplate` with the error message and a status code of NOT_FOUND
        Err(e) => (
            StatusCode::NOT_FOUND,
            IndexTemplate::error(e.to_string()),
            // Html(include_str!("../assets/static/404.html")),
        )
            .into_response(),
    }
}
