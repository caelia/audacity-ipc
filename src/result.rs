use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    Audacity(String),
    Connection(String),
    Warning(String),
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Error::Audacity(desc) => {
                write!(f, "Audacity Error: {}", desc)
            },
            Error::Connection(desc) => {
                write!(f, "Connection Error: {}", desc)
            },
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
