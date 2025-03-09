use crate::value::Value;
use serde_yaml::Value as Yaml;

impl From<Yaml> for Value {
    fn from(value: Yaml) -> Value {
        match value  {
            Yaml::Bool(true) => Value::Boolean(true),
            _ => Value::Boolean(false)
        }
    }
}


impl Into<Yaml> for Value {
    fn into(self) -> Yaml {
        match self  {
            Value::Boolean(value) => Yaml::Bool(value),
        }
    }
}
