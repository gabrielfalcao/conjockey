use crate::integer::Integer;
use crate::value::Value;
use serde_json::Number as JsonNumber;
use serde_json::Value as JsonValue;
use std::any::TypeId;

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
