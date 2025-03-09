use crate::value::Value;
use ::toml::Value as Toml;

#[test]
fn test_toml_boolean_false() {
    assert_eq!(
        Toml::Boolean(false),
        Into::<Toml>::into(Value::Boolean(false)),
    );
    assert_eq!(
        Value::Boolean(false),
        Into::<Value>::into(Toml::Boolean(false)),
    );
}

#[test]
fn test_toml_boolean_true() {
    assert_eq!(
        Toml::Boolean(true),
        Into::<Toml>::into(Value::Boolean(true)),
    );
    assert_eq!(
        Value::Boolean(true),
        Into::<Value>::into(Toml::Boolean(true)),
    );
}
