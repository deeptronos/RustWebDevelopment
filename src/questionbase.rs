use crate::*;

#[derive(Debug, thiserror::Error, Serialize)]
pub enum QuestionBaseErr {
    #[error("Question already exists: {0}")]
    QuestionExists(String),
    #[error("QuestionBase - IO failed: {0}")]
    QuestionBaseIoError(String),
    #[error("No Question")]
    NoQuestion,
    #[error("Question {0} doesn't exist")]
    QuestionDoesNotExist(String),
    #[error("Question payload unprocessable: {0}")]
    QuestionUnprocessable(String),
    #[error("Database error: {0}")]
    DatabaseError(String),
}

impl From<std::io::Error> for QuestionBaseErr {
    fn from(e: std::io::Error) -> Self {
        QuestionBaseErr::QuestionBaseIoError(e.to_string())
    }
}

impl From<sqlx::Error> for QuestionBaseErr {
    fn from(e: sqlx::Error) -> Self {
        QuestionBaseErr::DatabaseError(e.to_string())
    }
}

#[derive(Debug)]
pub struct QuestionBaseError {
    pub status: StatusCode,
    pub error: QuestionBaseErr,
}

impl Serialize for QuestionBaseError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let status: String = self.status.to_string();
        let mut state = serializer.serialize_struct("QuestionBaseError", 2)?;
        state.serialize_field("status", &status)?;
        state.serialize_field("error", &self.error)?;
        state.end()
    }
}

impl QuestionBaseError {
    pub fn response(status: StatusCode, error: QuestionBaseErr) -> Response {
        let error = QuestionBaseError { status, error };
        (status, Json(error)).into_response()
    }
}

#[derive(Debug)]
pub struct QuestionBase(pub Pool<Postgres>);

fn to_question(row: PgRow) -> Question {
    Question {
        id: row.get("id"),
        title: row.get("title"),
        body: row.get("body"),
        asker: row.get("asker"),
        // tags: None, // TODO
    }
}

impl QuestionBase {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        use std::env::var;

        let pwf = var("PG_PASSWORDFILE")?;
        let password = std::fs::read_to_string(pwf)?;
        let url = format!(
            "postgres://{}:{}@{}:5432/{}",
            var("PG_USER")?,
            password.trim(),
            var("PG_HOST")?,
            var("PG_DBNAME")?,
        );
        let pool = PgPool::connect(&url).await?;
        sqlx::migrate!().run(&pool).await?;
        Ok(QuestionBase(pool))
    }

    pub fn get_random(&self) -> Option<&Question> {
        todo!()
    }

    pub fn get<'a>(&'a self, index: &str) -> Result<&'a Question, QuestionBaseErr> {
        todo!()
    }
    pub async fn get_questions<'a>(&self) -> Result<Vec<Question>, QuestionBaseErr> {
        let questions = sqlx::query("SELECT * FROM example_questions;")
            .fetch_all(&self.0)
            .await?;
        let questions: Vec<Question> = questions.into_iter().map(|q| to_question(q)).collect();
        Ok(questions)
    }

    pub fn add(&mut self, _question: Question) -> Result<(), QuestionBaseErr> {
        todo!()
    }

    pub fn delete(&mut self, _index: &str) -> Result<(), QuestionBaseErr> {
        todo!()
    }

    pub fn update(&mut self, index: &str, question: Question) -> Result<(), QuestionBaseErr> {
        todo!()
    }
}
