use crate::value::Value;
use serde_json::Value as Json;

impl From<Json> for Value {
    fn from(value: Json) -> Value {
        match value {
            Json::Bool(value) => Value::Boolean(value),
            Json::String(value) => Value::String(value),
            Json::Number(value) => Value::Integer(value.into()),
            Json::Null | _ => Value::Null,
        }
    }
}

impl Into<Json> for Value {
    fn into(self) -> Json {
        match self {
            Value::Boolean(value) => Json::Bool(value),
            Value::String(value) => Json::String(value),
            Value::Integer(value) => Json::Number(value.into()),
            Value::Null | _ => Json::Null,
        }
    }
}
