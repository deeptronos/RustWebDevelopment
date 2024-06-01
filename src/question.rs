use crate::*;
// Importing necessary modules from the current crate
use crate::*;

// Defining a struct to represent a Question
/// A struct to represent a Question.
///
/// # Fields
///
/// * `id` - A unique identifier for the question.
/// * `title` - The title of the question.
/// * `body` - The body or content of the question.
/// * `asker` - The username of the person who asked the question.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Question {
    pub id: String,
    pub title: String,
    pub body: String,
    pub asker: String,
}

// Implementing a new method for the Question struct
impl Question {
    /// Creates a new instance of Question.
    ///
    /// # Arguments
    ///
    /// * `id` - A unique identifier for the question.
    /// * `title` - The title of the question.
    /// * `body` - The body or content of the question.
    /// * `asker` - The username of the person who asked the question.
    ///
    /// # Returns
    ///
    /// A new instance of Question.
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

// Implementing IntoResponse trait for &Question
impl IntoResponse for &Question {
    /// Converts a reference to Question into a Response.
    ///
    /// # Returns
    ///
    /// A Response with a status code of OK and the question data in JSON format.
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(&self)).into_response() // TODO requires From<&Question> for String??
    }
}
