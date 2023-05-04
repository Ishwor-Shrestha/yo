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
}

#[derive(Debug, Error)]
pub struct Error {
    message: String,
    pub code: i32,
    kind: ErrorKind,
    origin: String,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} {{\n{} ({})\n{}\n}}",
            self.kind, self.message, self.code, self.origin
        )
    }
}

impl Error {
    pub fn new<T: std::error::Error>(
        message: String,
        code: i32,
        kind: ErrorKind,
        source: T,
    ) -> Self {
        let formatted_source = format!("{:?}", source);
        let source_name = match formatted_source.split_whitespace().nth(0) {
            Some(x) => x.to_string(),
            None => formatted_source,
        };
        Error {
            message,
            code,
            kind,
            origin: format!("{source_name} ({})", source.to_string()),
        }
    }

    pub fn new_sourceless(message: String, code: i32, kind: ErrorKind) -> Self {
        Error {
            message,
            code,
            kind,
            origin: "None".to_string(),
        }
    }

    pub fn empty() -> Self {
        Error {
            message: "".to_string(),
            code: -1,
            kind: ErrorKind::Unknown,
            origin: "None".to_string(),
        }
    }
}
