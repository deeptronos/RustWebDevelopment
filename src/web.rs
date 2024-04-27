use crate::*;

#[derive(Template)]
#[template(path = "index.html")]

pub struct IndexTemplate<'a> {
    question: &'a Question,
}

pub async fn handler_index(State(question_base): State<Arc<RwLock<QuestionBase>>>) -> Response {
    match question_base.read().await.get_random() {
        Some(question) => (StatusCode::OK, IndexTemplate { question }).into_response(),
        None => (
            StatusCode::NOT_FOUND,
            Html(include_str!("../res/static/404.html")),
        )
            .into_response(), // TODO ok?
    }
}
