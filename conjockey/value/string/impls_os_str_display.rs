use crate::value::Value;
use std::ffi::{OsStr, OsString};

impl From<OsString> for Value {
    fn from(os: OsString) -> Value {
        Value::String(os.display().to_string())
    }
}

impl From<&OsStr> for Value {
    fn from(os: &OsStr) -> Value {
        Value::String(os.display().to_string())
    }
}
