use crate::float::Float;
use crate::value::Value;
use ::plist::Value as PlistValue;

#[test]
fn test_plist_float() {
    assert_eq!(
        PlistValue::Real(2.718281828459045f64),
        Into::<PlistValue>::into(Value::Float(Float(2.718281828459045f64))),
    );
    assert_eq!(
        Value::Float(Float(2.718281828459045f64)),
        Into::<Value>::into(PlistValue::Real(2.718281828459045f64)),
    );
}
