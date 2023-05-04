use thiserror::Error;

#[derive(Debug)]
pub enum ErrorKind {
    Io,
    Open,
    Parse,
    Unknown,
    Serialize,
    Deserialize,
    FileSystem,
    Project,
}

#[derive(Debug, Error)]
pub struct Error {
    pub message: String,
    pub code: i32,
    pub kind: ErrorKind,
    pub origin: String,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} {{\n    {} ({})\n    {}\n}}",
            self.kind, self.message, self.code, self.origin
        )
    }
}

impl Error {
    pub fn new(message: String) -> Error {
        Error {
            message,
            code: exitcode::TEMPFAIL,
            kind: ErrorKind::Unknown,
            origin: String::from("None"),
        }
    }

    pub fn code(mut self, code: i32) -> Error {
        self.code = code;
        self
    }

    pub fn kind(mut self, kind: ErrorKind) -> Error {
        self.kind = kind;
        self
    }

    pub fn source<T: std::error::Error>(mut self, source: T) -> Error {
        let formatted_source = format!("{:?}", source);
        let source_name = match formatted_source.split_whitespace().nth(0) {
            Some(x) => x.to_string(),
            None => formatted_source,
        };

        self.origin = format!("{source_name} ({})", source.to_string());
        self
    }
}
