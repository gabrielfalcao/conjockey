use crate::value::Value;
use serde_json::Value as Json;

impl From<Json> for Value {
    fn from(value: Json) -> Value {
        match value  {
            Json::Bool(true) => Value::Boolean(true),
            _ => Value::Boolean(false)
        }
    }
}


impl Into<Json> for Value {
    fn into(self) -> Json {
        match self  {
            Value::Boolean(value) => Json::Bool(value),
        }
    }
}
