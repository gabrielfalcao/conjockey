use crate::integer::Integer;
use crate::value::Value;
use serde_yaml::Number as YamlNumber;
use serde_yaml::Value as YamlValue;
use std::any::TypeId;

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

impl Into<YamlNumber> for Value {
    fn into(self) -> YamlNumber {
        match self {
            Value::Integer(value) => Into::<YamlNumber>::into(value),
            _ => panic!(
                "can not convert {:#?} to {:#?}",
                self,
                TypeId::of::<Value>()
            ),
        }
    }
}

impl Into<YamlValue> for Integer {
    fn into(self) -> YamlValue {
        YamlValue::Number(YamlNumber::from(self.to_signed_i64()))
    }
}
