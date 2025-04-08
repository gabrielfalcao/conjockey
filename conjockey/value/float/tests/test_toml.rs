use crate::float::Float;
use crate::value::Value;
use ::toml::Value as Toml;

#[test]
fn test_toml_float() {
    assert_eq!(
        Toml::Float(2.718281828459045f64),
        Into::<Toml>::into(Value::Float(Float(2.718281828459045f64))),
    );
    assert_eq!(
        Value::Float(Float(2.718281828459045f64)),
        Into::<Value>::into(Toml::Float(2.718281828459045f64)),
    );
}

#[test]
fn test_toml_float_from_option() {
    assert_eq!(
        Toml::Float(2.718281828459045f64),
        Into::<Toml>::into(Value::from(Some(2.718281828459045f64)))
    );
}
