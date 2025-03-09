use crate::value::Value;
use std::borrow::Cow;

impl From<String> for Value {
    fn from(value: String) -> Value {
        Value::String(value)
    }
}

impl From<&String> for Value {
    fn from(value: &String) -> Value {
        Value::String(value.to_string())
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Value {
        Value::String(value.to_string())
    }
}

impl From<Cow<'static, str>> for Value {
    fn from(cow: Cow<'static, str>) -> Value {
        match cow {
            Cow::Borrowed(p) => Value::String(Into::<String>::into(p)),
            Cow::Owned(p) => Value::String(Into::<String>::into(p)),
        }
    }
}
