use crate::float::Float;
use crate::value::Value;
use ::serde_json::Number as JsonNumber;
use ::serde_json::Value as JsonValue;

#[test]
fn test_json_float() {
    assert_eq!(
        JsonValue::Number(JsonNumber::from(2.718281828459045f64)),
        Into::<JsonValue>::into(Value::Float(Float(2.718281828459045f64))),
    );
    assert_eq!(
        Value::Float(Float(2.718281828459045f64)),
        Into::<Value>::into(JsonValue::Number(JsonNumber::from(2.718281828459045f64))),
    );
}

#[test]
fn test_json_float_from_option_float() {
    assert_eq!(
        JsonValue::Number(JsonNumber::from(2.718281828459045f64)),
        Into::<JsonValue>::into(Some(Float(2.718281828459045f64))),
    );
}

#[test]
fn test_json_float_from_option_value_float() {
    assert_eq!(
        JsonValue::Number(JsonNumber::from(2.718281828459045f64)),
        Into::<JsonValue>::into(Some(Value::Float(Float(2.718281828459045f64)))),
    );
}
