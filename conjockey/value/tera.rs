use crate::value::Value;
use ::tera::Value as Tera;

impl From<Tera> for Value {
    fn from(value: Tera) -> Value {
        match value  {
            Tera::Bool(value) => Value::Boolean(value),
            Tera::String(value) => Value::String(value),
            _ => Value::Boolean(false)
        }
    }
}
