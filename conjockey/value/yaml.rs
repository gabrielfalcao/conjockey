use crate::float::Float;
use crate::integer::Integer;
use crate::value::Value;
use ::serde_yaml::Number as YamlNumber;
use ::serde_yaml::Value as YamlValue;
use std::any::TypeId;

impl From<YamlValue> for Value {
    fn from(value: YamlValue) -> Value {
        match value {
            YamlValue::Bool(value) => Value::Boolean(value),
            YamlValue::String(value) => Value::String(value),
            YamlValue::Number(value) => {
                if value.as_f64().unwrap_or_default() == value.as_f64().unwrap_or_default().round() {
                    Value::Integer(Integer(value.as_i64().unwrap()))
                } else {
                    Value::Float(Float(value.as_f64().unwrap()))
                }
            }
            YamlValue::Null | _ => Value::Null,
        }
    }
}

impl Into<YamlValue> for Value {
    fn into(self) -> YamlValue {
        match self {
            Value::Boolean(value) => YamlValue::Bool(value),
            Value::Float(value) => YamlValue::Number(YamlNumber::from(value.as_f64())),
            Value::Integer(value) => YamlValue::Number(YamlNumber::from(value.as_i64())),
            Value::String(value) => YamlValue::String(value),
            Value::Null | _ => YamlValue::Null,
        }
    }
}

impl Into<YamlNumber> for Value {
    fn into(self) -> YamlNumber {
        match self {
            Value::Float(value) => Into::<YamlNumber>::into(value),
            Value::Integer(value) => Into::<YamlNumber>::into(value),
            _ => panic!(
                "can not convert {:#?} to {:#?}",
                self,
                TypeId::of::<Value>()
            ),
        }
    }
}

impl From<YamlNumber> for Integer {
    fn from(value: YamlNumber) -> Integer {
        match value.as_i64() {
            Some(value) => Integer(value),
            None => panic!(
                "can not convert {:#?} into i64 for conjockey::Integer",
                value
            ),
        }
    }
}
impl From<Integer> for YamlNumber {
    fn from(value: Integer) -> YamlNumber {
        match value.as_signed_i64() {
            Some(value) => YamlNumber::from(value),
            None => panic!(
                "can not convert {:#?} into i64 for {:#?}",
                value,
                TypeId::of::<YamlNumber>()
            ),
        }
    }
}

impl Into<YamlValue> for Integer {
    fn into(self) -> YamlValue {
        YamlValue::Number(YamlNumber::from(self.to_signed_i64()))
    }
}

impl From<Float> for YamlNumber {
    fn from(value: Float) -> YamlNumber {
        YamlNumber::from(value.as_f64())
        // match YamlNumber::from(value.as_f64()) {
        //     Some(value) => value,
        //     None => panic!(
        //         "can not convert {:#?} into {:#?}",
        //         value.as_f64(),
        //         TypeId::of::<YamlNumber>()
        //     ),
        // }
    }
}
