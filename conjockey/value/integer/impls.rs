use crate::integer::Integer;
use crate::value::Value;

impl From<Integer> for Value {
    fn from(value: Integer) -> Value {
        Value::Integer(value)
    }
}

impl From<&Integer> for Value {
    fn from(value: &Integer) -> Value {
        Value::Integer(Integer(value.to_signed()))
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Value {
        Value::Integer(Into::<Integer>::into(value))
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Value {
        Value::Integer(Into::<Integer>::into(value))
    }
}

impl From<i16> for Value {
    fn from(value: i16) -> Value {
        Value::Integer(Into::<Integer>::into(value))
    }
}

impl From<i8> for Value {
    fn from(value: i8) -> Value {
        Value::Integer(Into::<Integer>::into(value))
    }
}
