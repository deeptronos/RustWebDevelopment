use crate::*;

#[derive(Debug, thiserror::Error, Serialize)]
pub enum QuestionBaseErr {
    #[error("Question already exists: {0}")]
    QuestionExists(String),

    /// Indicates an IO failure during a QuestionBase operation.
    #[error("QuestionBase - IO failed: {0}")]
    QuestionBaseIoError(String),

    /// Indicates that no question was found in the database.
    #[error("No Question")]
    NoQuestion,

    /// Indicates that a specified question does not exist in the database.
    #[error("Question {0} doesn't exist")]
    QuestionDoesNotExist(String),

    /// Indicates that the payload of a question is unprocessable.
    #[error("Question payload unprocessable: {0}")]
    QuestionUnprocessable(String),

    /// Indicates a database error.
    #[error("Database error: {0}")]
    DatabaseError(String),
}
/// Implements `From` trait for converting a standard `std::io::Error` into a custom `QuestionBaseErr`.
impl From<std::io::Error> for QuestionBaseErr {
    fn from(e: std::io::Error) -> Self {
        QuestionBaseErr::QuestionBaseIoError(e.to_string())
    }
}
/// Implements `From` trait for converting a standard `sqlx::Error` into a custom `QuestionBaseErr`.
impl From<sqlx::Error> for QuestionBaseErr {
    fn from(e: sqlx::Error) -> Self {
        QuestionBaseErr::DatabaseError(e.to_string())
    }
}
/// Represents an error that occurred while processing a question in the `QuestionBase` struct.
#[derive(Debug)]
pub struct QuestionBaseError {
    /// The HTTP status code associated with this error.
    pub status: StatusCode,

    /// The custom error type representing the specific issue encountered during question processing.
    pub error: QuestionBaseErr,
}
/// Implements `Serialize` trait for serializing a `QuestionBaseError` struct into JSON format.
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
/// Implements a method for creating a response from a `QuestionBaseError`.
impl QuestionBaseError {
    pub fn response(status: StatusCode, error: QuestionBaseErr) -> Response {
        let error = QuestionBaseError { status, error };
        (status, Json(error)).into_response()
    }
}

/// An instance of a persistant PostgreSQL database in which to store [`Question`]s.
#[derive(Debug)]
pub struct QuestionBase(pub Pool<Postgres>);

impl QuestionBase {
    /// Converts a PostgreSQL `PgRow` into a [`Question`].
    async fn to_question(&self, row: &PgRow) -> Result<Question, sqlx::Error> {
        Ok(Question {
            id: row.get("id"),
            title: row.get("title"),
            body: row.get("body"),
            asker: row.get("asker"),
            // tags: None, // TODO
        })
    }

    /// Creates a new instance of `QuestionBase` by connecting to a PostgreSQL database using environment variables.
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

    /// Inserts a new set of tags into the database for a given question.
    async fn insert_tags(
        &mut self,
        tx: &mut PgConnection, // Pass in an active transaction to avoid establishing a new one.
        id: &str,              // The id of the question that the tags will be attached to.
        tags: &Option<HashSet<String>>, // The optional set of tags to insert. If none, no tags are inserted.
    ) -> Result<(), sqlx::Error> {
        todo!()
    }

    /// Retrieves a random [`Question`] from the database.
    pub async fn get_random(&self) -> Result<Question, QuestionBaseErr> {
        let row = sqlx::query(r#"SELECT * FROM questions ORDER BY RANDOM () LIMIT 1;"#)
            .fetch_one(&self.0)
            .await?;

        let question = self.to_question(&row).await?;
        Ok(question)
    }
    /// Retrieves a specific [`Question`] from the database based on its ID.
    pub async fn get<'a>(&'a self, index: &str) -> Result<Question, QuestionBaseErr> {
        let row = sqlx::query(r#"SELECT * FROM questions WHERE id = $1;"#)
            .bind(index)
            .fetch_one(&self.0)
            .await?;

        let question = self.to_question(&row).await?;
        Ok(question)
    }

    /// Retrieves all [`Question`]s from the database.
    pub async fn get_questions<'a>(&self) -> Result<Vec<Question>, QuestionBaseErr> {
        let rows = sqlx::query(r#"SELECT * FROM questions;"#)
            .fetch_all(&self.0)
            .await?;
        let mut questions: Vec<Question> = Vec::with_capacity(rows.len());
        for q in rows.iter() {
            questions.push(self.to_question(q).await?);
        }
        Ok(questions)
    }

    /// Inserts a new [`Question`] into the database, returning an error if a question with the same ID already exists.
    pub async fn add(&mut self, question: Question) -> Result<(), QuestionBaseErr> {
        let mut tx = Pool::begin(&self.0).await?;
        let result = sqlx::query(
            r#"INSERT INTO questions
                (id, title, body, asker)
                VALUES ($1, $2, $3, $4);"#,
        )
        .bind(&question.id)
        .bind(&question.title)
        .bind(&question.body)
        .bind(&question.asker)
        .execute(&mut *tx)
        .await;
        result.map_err(|e| {
            if let sqlx::Error::Database(ref dbe) = e {
                if let Some("23505") = dbe.code().as_deref() {
                    return QuestionBaseErr::QuestionExists(question.id.to_string());
                }
            }
            QuestionBaseErr::DatabaseError(e.to_string())
        })?;
        // Self::insert_tags(&mut tx, &joke.id, &joke.tags).await?; TODO
        Ok(tx.commit().await?)
    }
    /// Deletes a [`Question`] from the database based on its ID.
    pub async fn delete(&mut self, index: &str) -> Result<(), QuestionBaseErr> {
        let mut tx = Pool::begin(&self.0).await?;
        sqlx::query(r#"DELETE FROM questions WHERE id = $1;"#)
            .bind(index)
            .execute(&mut *tx)
            .await?;
        // sqlx::query(r#"DELETE FROM tags WHERE id = $1;"#) TODO
        //     .bind(index)
        //     .execute(&mut *tx)
        //     .await?;
        Ok(tx.commit().await?)
    }

    /// Updates an existing [`Question`] in the database based on its ID, returning an error if the question does not exist.
    pub async fn update(&mut self, index: &str, question: Question) -> Result<(), QuestionBaseErr> {
        let mut tx = Pool::begin(&self.0).await?;
        let q = sqlx::query(
            r#"UPDATE questions SET
            (id, title, body, asker)
            VALUES ($1, $2, $3, $4);"#,
        );
        q.bind(&question.id)
            .bind(&question.title)
            .bind(&question.body)
            .bind(&question.asker)
            .execute(&mut *tx)
            .await?;
        // sqlx::query(r#"DELETE FROM tags WHERE id = $1;"#)
        //     .bind(index)
        //     .execute(&mut *tx)
        //     .await?;
        // Self::insert_tags(&mut tx, &question.id, &question.tags).await?;
        Ok(tx.commit().await?)
    }
}
