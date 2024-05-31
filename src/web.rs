use crate::*;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
    question: Option<&'a Question>,
    tags: Option<String>,
    stylesheet: &'static str,
    error: Option<String>,
}

impl<'a> IndexTemplate<'a> {
    fn question(question: &'a Question) -> Self {
        Self {
            question: Some(question),
            tags: None,
            stylesheet: "/question.css",
            error: None,
        }
    }
    fn error(error: String) -> Self {
        Self {
            question: None,
            tags: None,
            stylesheet: "/question.css",
            error: Some(error),
        }
    }
}

pub async fn handler_index(State(question_base): State<Arc<RwLock<QuestionBase>>>) -> Response {
    match question_base.read().await.get_random().await {
        Ok(question) => (StatusCode::OK, IndexTemplate::question(&question)).into_response(),
        Err(e) => (
            StatusCode::NOT_FOUND,
            IndexTemplate::error(e.to_string()),
            // Html(include_str!("../assets/static/404.html")),
        )
            .into_response(),
    }
}
