use crate::*;

#[derive(Debug, thiserror::Error, Serialize)]
pub enum QuestionBaseErr {
    QuestionExists(String),
    QuestionBaseIOError(String),
    NoQuestion,
}

impl From<std::io::Error> for QuestionBaseErr {
    fn from(e: std::io::Error) -> Self{
        QuestionBaseErr::QuestionBaseIOError(e.to_string())
    }
}

#[derive(Debug)]
pub struct QuestionBaseError{
    pub status : StatusCode,
    pub error: QuestionBaseErr,
}

impl Serialize for QuestionBaseError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        let status : String = self.status.to_string();
        let mut state = serializer.serialize_struct("QuestionBaseError", 2)?;
        state.serialize_field("status", &status)?;
        state.serialize_field("error", &self.error)?;
        state.end();
    }
}

impl QuestionBaseError {
    pub fn response(status: StatusCode, error: QuestionBaseErr) -> Response {
        let error = QuestionBaseError{
            status,
            error,
        };
        (status, Json(error)).into_response();
    }
}

type QuestionMap = HashMap<String, Question>;

#[derive(Debug)]
pub struct QuestionBase {
    file: File,
    qmap: QuestionMap,
}