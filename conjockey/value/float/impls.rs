use crate::float::Float;
use crate::value::Value;

impl From<Float> for Value {
    fn from(value: Float) -> Value {
        Value::Float(value)
    }
}

impl From<&Float> for Value {
    fn from(value: &Float) -> Value {
        Value::Float(Float(value.to_signed()))
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Value {
        Value::Float(Into::<Float>::into(value))
    }
}
