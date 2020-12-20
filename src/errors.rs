use std::{error, fmt, result};

pub type Result<T> = result::Result<T,Error>;

#[derive(Debug)]
pub enum Error {
    NoDateTime(String),
    BadDateCalc(String),
    ParseDateTime,
    SerdeError(serde_json::Error),
}

impl fmt::Display for Error{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{:?}", self)
    }
}

impl error::Error for Error{
}

// converters
impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error)->Self{ Error::SerdeError(err) }
}