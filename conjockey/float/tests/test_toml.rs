use crate::value::Value;
use toml::Value as Toml;

#[test]
fn test_serde_toml_float() {
    assert_eq!(
        Toml::Float(5.0),
        Into::<Toml>::into(Value::Float(Float(5.0))),
    );
    assert_eq!(
        Value::Float(Float(5.0)),
        Into::<Value>::into(Float(5.0)),
    );
}
