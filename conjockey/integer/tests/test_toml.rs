use crate::value::Value;
use toml::Value as Toml;

#[test]
fn test_serde_toml_integer() {
    assert_eq!(
        Toml::Integer(5),
        Into::<Toml>::into(Value::Integer(Integer(5))),
    );
    assert_eq!(
        Value::Integer(Integer(5)),
        Into::<Value>::into(Value::Integer(Integer(5))),
    );
}
