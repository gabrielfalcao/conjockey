use crate::float::Float;
use crate::value::Value;
use ::tera::Number as TeraNumber;
use ::tera::Value as TeraValue;

#[test]
fn test_tera_float() {
    assert_eq!(
        TeraValue::Number(TeraNumber::from(2.718281828459045)),
        Into::<TeraValue>::into(Value::Float(Float(2.718281828459045))),
    );
    assert_eq!(
        Value::Float(Float(2.718281828459045f64)),
        Into::<Value>::into(TeraValue::Number(TeraNumber::from(2.718281828459045f64))),
    );
}

#[test]
fn test_tera_float_from_option_float() {
    assert_eq!(
        TeraValue::Number(TeraNumber::from(2.718281828459045f64)),
        Into::<TeraValue>::into(Some(Float(2.718281828459045f64))),
    );
}

#[test]
fn test_tera_float_from_option_value_float() {
    assert_eq!(
        TeraValue::Number(TeraNumber::from(2.718281828459045f64)),
        Into::<TeraValue>::into(Some(Value::Float(Float(2.718281828459045f64)))),
    );
}
