use crate::value::Value;
use toml::Value as Toml;

#[test]
fn test_toml_null() {
    assert_eq!(
        Toml::Boolean(false),
        Into::<Toml>::into(Value::Null),
    );
}

#[test]
fn test_toml_null_from_option() {
    assert_eq!(
        Toml::Boolean(false),
        Into::<Toml>::into(Value::from(None::<bool>)),
    );
}
