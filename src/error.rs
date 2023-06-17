use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    Audacity(String),
    Connection(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
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
