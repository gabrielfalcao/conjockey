use crate::value::Value;
use tera::Value as Tera;

impl From<Tera> for Value {
    fn from(value: Tera) -> Value {
        match value {
            Tera::Bool(value) => Value::Boolean(value),
            Tera::String(value) => Value::String(value),
            Tera::Number(value) => Value::Integer(value.into()),
            Tera::Null | _ => Value::Null,
        }
    }
}

impl Into<Tera> for Value {
    fn into(self) -> Tera {
        match self {
            Value::Boolean(value) => Tera::Bool(value),
            Value::String(value) => Tera::String(value),
            Value::Integer(value) => Tera::Number(value.into()),
            Value::Null | _ => Tera::Null,
        }
    }
}
