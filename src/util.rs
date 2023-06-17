use std::path::PathBuf;
use crate::{conn::Connection, result::Error};

pub fn check_status(con: &Connection) -> Result<&str, Error> {
    todo!()
}

pub fn edit_file(con: &Connection, file: &PathBuf) -> Result<(), Error> {
    todo!()
}

pub fn apply_macro(con: &Connection, mac: String, params: Vec<String>) -> Result<(), Error> {
    todo!()
}
