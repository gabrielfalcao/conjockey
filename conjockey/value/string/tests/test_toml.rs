use crate::value::Value;
use toml::Value as Toml;

#[test]
fn test_serde_toml_string() {
    assert_eq!(
        Toml::String(String::from("String")),
        Into::<Toml>::into(Value::String(String::from("String"))),
    );
    assert_eq!(
        Value::String(String::from("String")),
        Into::<Value>::into(Toml::String(String::from("String"))),
    );
}
