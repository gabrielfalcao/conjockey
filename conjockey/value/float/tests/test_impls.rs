use crate::float::Float;
use crate::value::Value;

#[test]
fn test_float_into_value_float() {
    assert_eq!(Into::<Value>::into(2.718281828459045f64), Value::Float(Float(2.718281828459045f64)));
}


#[test]
fn test_value_float_into_float() {
    assert_eq!(Value::Float(Float(2.718281828459045f64)), Into::<Value>::into(2.718281828459045f64));
}


#[test]
fn test_option_float_into_value_float() {
    assert_eq!(Into::<Value>::into(Some(2.718281828459045f64)), Value::Float(Float(2.718281828459045f64)));
}
