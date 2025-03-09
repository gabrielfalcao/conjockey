use crate::value::Value;

impl <T> From<Option<T>> for Value where T: Into<Value> {
    fn from(value: Option<T>) -> Value {
        match value {
            None => Value::Null,
            Some(value) => Into::<Value>::into(value)
        }
    }
}
