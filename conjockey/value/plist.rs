use crate::value::Value;
use ::plist::Value as Plist;

impl From<Plist> for Value {
    fn from(value: Plist) -> Value {
        match value {
            Plist::Boolean(value) => Value::Boolean(value),
            Plist::String(value) => Value::String(value),
            Plist::Integer(value) => Value::Integer(value.into()),
            _ => Value::Null,
        }
    }
}

impl Into<Plist> for Value {
    fn into(self) -> Plist {
        match self {
            Value::Boolean(value) => Plist::Boolean(value),
            Value::String(value) => Plist::String(value),
            Value::Integer(value) => Plist::Integer(value.into()),
            Value::Null | _ => Plist::Boolean(false),
        }
    }
}
