use crate::*;

// #[derive(Debug,  thiserror::Error, Serialize)]
// pub enum QuestionBaseErr {
//     QuestionExists(String),
//     QuestionBaseIoError(String),
//     NoQuestion,
// }

// impl From<std::io::Error> for QuestionBaseErr {
//     fn from(e: std::io::Error) -> Self{
//         QuestionBaseErr::QuestionBaseIoError(e.to_string())
//     }
// }

#[derive(Debug, thiserror::Error, Serialize)]
pub enum QuestionBaseErr {
    #[error("Question already exists: {0}")]
    QuestionExists(String),
    #[error("QuestionBase - IO failed: {0}")]
    QuestionBaseIoError(String),
    #[error("No Question")]
    NoQuestion,
}

impl From<std::io::Error> for QuestionBaseErr {
    fn from(e: std::io::Error) -> Self {
        QuestionBaseErr::QuestionBaseIoError(e.to_string())
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
        state.end()
    }
}

impl QuestionBaseError {
    pub fn response(status: StatusCode, error: QuestionBaseErr) -> Response {
        let error = QuestionBaseError{
            status,
            error,
        };
        (status, Json(error)).into_response()
    }
}

type QuestionMap = HashMap<String, Question>;

#[derive(Debug)]
pub struct QuestionBase {
    file: File,
    qmap: QuestionMap,
}

impl QuestionBase {
    pub fn new <P: AsRef<std::path::Path>>(db_path: P) -> Result<Self, std::io::Error> {
        let mut file = File::create_new(&db_path)
                    .and_then( |mut f|{
                        let questionmap : QuestionMap = HashMap::new();
                        let json = serde_json::to_string(&questionmap).unwrap();
                        f.write_all(json.as_bytes())?;
                        f.sync_all()?;
                        f.rewind()?;
                        Ok(f)
                    })
                    .or_else( |e|{
                        if e.kind() == ErrorKind::AlreadyExists {
                            File::options().read(true).write(true).open(&db_path)
                        }else{
                            Err(e)
                        }
                    })?;
        let json = std::io::read_to_string(&mut file)?;
        let qmap = serde_json::from_str(&json).map_err(|e| std::io::Error::new(ErrorKind::InvalidData, e))?;
        Ok(Self {file, qmap})
    }

    pub fn get_random(&self) -> Option<&Question> {
        fastrand::choice(self.qmap.iter()).map(|x| x.1)
    }

    pub fn get<'a>(&'a self, index: &str) -> Option<&'a Question> {
        self.qmap.get(index)
    }

    pub fn write_questions(&mut self) -> Result<(), std::io::Error> {
        let json = serde_json::to_string(&self.qmap).unwrap();
        self.file.rewind()?;
        self.file.set_len(0)?;
        self.file.write_all(json.as_bytes())?;
        self.file.sync_all()
    }

    pub fn add(&mut self, question : Question) -> Result<(), QuestionBaseErr> {
        let id = question.id.clone();
        if self.qmap.get(&id).is_some() {
            return Err(QuestionBaseErr::QuestionExists(id));
        }
        self.qmap.insert(id, question);
        self.write_questions()?;
        Ok(())
    }
}

impl IntoResponse for &QuestionBase {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(&self.qmap)).into_response()
    }
}