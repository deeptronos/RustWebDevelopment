use crate::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Question {
    pub id: String,
    pub title: String,
    pub body: String,
    pub asker: String,
}

impl Question {
    pub fn new(id: &str, title: &str, body: &str, asker: &str) -> Self {
        let id = id.into();
        let title = title.into();
        let body = body.into();
        let asker = asker.into();

        Self {
            id,
            title,
            body,
            asker,
        }
    }
}

impl IntoResponse for &Question {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(&self)).into_response() // TODO requires From<&Question> for String??
    }
}
