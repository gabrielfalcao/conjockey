use crate::value::Value;
use ::toml::Value as Toml;

impl From<Toml> for Value {
    fn from(value: Toml) -> Value {
        match value  {
            Toml::Boolean(value) => Value::Boolean(value),
            Toml::String(value) => Value::String(value),
            _ => Value::Boolean(false)
        }
    }
}


impl Into<Toml> for Value {
    fn into(self) -> Toml {
        match self  {
            Value::Boolean(value) => Toml::Boolean(value),
            Value::String(value) => Toml::String(value),
        }
    }
}
