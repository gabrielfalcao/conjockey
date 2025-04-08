use crate::float::Float;
use crate::integer::Integer;
use crate::value::Value;
use ::serde_json::Number as JsonNumber;
use ::serde_json::Value as JsonValue;
use std::any::TypeId;

impl From<JsonValue> for Value {
    fn from(value: JsonValue) -> Value {
        match value {
            JsonValue::Bool(value) => Value::Boolean(value),
            JsonValue::String(value) => Value::String(value),
            JsonValue::Number(value) => {
                if value.as_f64().unwrap_or_default() == value.as_f64().unwrap_or_default().round() {
                    Value::Integer(Integer(value.as_i64().unwrap()))
                } else {
                    Value::Float(Float(value.as_f64().unwrap()))
                }
            }
            JsonValue::Null | _ => Value::Null,
        }
    }
}

impl Into<JsonValue> for Value {
    fn into(self) -> JsonValue {
        match self {
            Value::Boolean(value) => JsonValue::Bool(value),
            Value::Float(value) => JsonValue::Number(JsonNumber::from_f64(value.as_f64()).unwrap()),
            Value::Integer(value) => {
                JsonValue::Number(JsonNumber::from_i128(value.as_i64() as i128).unwrap())
            }
            Value::String(value) => JsonValue::String(value),
            Value::Null | _ => JsonValue::Null,
        }
    }
}

impl From<JsonNumber> for Integer {
    fn from(value: JsonNumber) -> Integer {
        match value.as_i64() {
            Some(value) => Integer(value),
            None => panic!(
                "can not convert {:#?} into i64 for conjockey::Integer",
                value
            ),
        }
    }
}
impl From<Integer> for JsonNumber {
    fn from(value: Integer) -> JsonNumber {
        match value.as_signed_i64() {
            Some(value) => JsonNumber::from(value),
            None => panic!(
                "can not convert {:#?} into i64 for {:#?}",
                value,
                TypeId::of::<JsonNumber>()
            ),
        }
    }
}

impl Into<JsonNumber> for Value {
    fn into(self) -> JsonNumber {
        match self {
            Value::Integer(value) => Into::<JsonNumber>::into(value),
            _ => panic!(
                "can not convert {:#?} to {:#?}",
                self,
                TypeId::of::<Value>()
            ),
        }
    }
}

impl Into<JsonValue> for Integer {
    fn into(self) -> JsonValue {
        JsonValue::Number(JsonNumber::from(self.to_signed_i64()))
    }
}

impl From<Float> for JsonNumber {
    fn from(value: Float) -> JsonNumber {
        match JsonNumber::from_f64(value.as_f64()) {
            Some(value) => value,
            None => panic!(
                "can not convert {:#?} into {:#?}",
                value.as_f64(),
                TypeId::of::<JsonNumber>()
            ),
        }
    }
}
