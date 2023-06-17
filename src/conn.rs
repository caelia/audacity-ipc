use std::path::PathBuf;
use std::fs::File;
use std::file::{open, options};
use crate::result::*;

enum Channel {
    Path(PathBuf),
    Pipe(File),
}

pub struct Connection {
    from: Channel,
    to: Channel,
}

impl Connection {
    pub fn new(from: PathBuf, to: PathBuf) -> Self {
        Connection { Channel::Path(from), Channel::Path(to) }
    }

    pub fn connect(&mut self) -> Result<()> {
        match (self.from, self.to) {
            (Channel::Pipe(_), Channel::Pipe(_)) => {
                return Err(Error::Warning(String::from("Already connected")));
            },
            (Channel::Pipe(_), _) | (_, Channel::Pipe(_)) => {
                return Err(Error::Connection(String::from("Connection struct not properly initialized")));
            },
            (Channel::Path(path_from), Channel::Path(path_to)) => {
                match (path_from.exists(), path_to.exists()) {
                    (false, false) => {
                        return Err(Error::Connection(String::from("pipes to & from Audacity do not exist")));
                    },
                    (false, _) => {
                        return Err(Error::Connection(String::from("pipe from Audacity does not exist")));
                    },
                    (_, false) => {
                        return Err(Error::Connection(String::from("pipe to Audacity does not exist"));
                    },
                    (_, _) => (),
                }
                match (path_from.is_file(), path_to.is_file()) {
                    (false, false) => {
                        return Err(Error::Connection(String::from("pipes to & from Audacity are not regular files")));
                    },
                    (false, _) => {
                        return Err(Error::Connection(String::from("pipe from Audacity is not a regular file")));
                    },
                    (_, false) => {
                        return Err(Error::Connection(String::from("pipe to Audacity is not a regular file"));
                    },
                    (_, _) => (),
                }
                match open(path_from) {
                    Ok(pipe_from) => {
                        self.from = Channel::Pipe(pipe_from);
                    },
                    Err(_) => {
                        return Err(Err::Connection(String::from("failed to open pipe from Audacity")));
                    },
                }
                match options().append(true).open(path_to) {
                    Ok(pipe_to) => {
                        self.to = Channel::Pipe(pipe_to);
                    },
                    Err(_) {
                        return Err(Err::Connection(String::from("failed to open pipe from Audacity")));
                    },
                }
            },
        }
        Ok(())
    }

    pub fn send(&self, message: String) -> Result<()> {
        todo!()
    }

    pub fn recv(&self) -> Result<String> {
        todo!()
    }
}
