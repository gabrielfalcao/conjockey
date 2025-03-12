use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use std::fmt::Display;


#[derive(Debug, Clone)]
pub enum Error {
    TryFromError(String),
}

impl Serialize for Error {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("Error", 2)?;
        s.serialize_field("variant", &self.variant())?;
        s.serialize_field("message", &format!("{}", self))?;
        s.end()
    }
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            self.variant(),
            match self {
                Self::TryFromError(s) => format!("{}", s),
            }
        )
    }
}

impl Error {
    pub fn variant(&self) -> String {
        match self {
            Error::TryFromError(_) => "TryFromError",
        }
        .to_string()
    }
}

impl std::error::Error for Error {}
pub type Result<T> = std::result::Result<T, Error>;
