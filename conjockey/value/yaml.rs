use crate::value::Value;
use serde_yaml::Value as Yaml;

impl From<Yaml> for Value {
    fn from(value: Yaml) -> Value {
        match value {
            Yaml::Bool(value) => Value::Boolean(value),
            Yaml::String(value) => Value::String(value),
            Yaml::Null | _ => Value::Null,
        }
    }
}

impl Into<Yaml> for Value {
    fn into(self) -> Yaml {
        match self {
            Value::Boolean(value) => Yaml::Bool(value),
            Value::String(value) => Yaml::String(value),
            Value::Null => Yaml::Null,
        }
    }
}
