use crate::value::Value;
use tera::Value as Toml;

#[test]
fn test_serde_tera_string() {
    assert_eq!(
        Toml::String(String::from("String")),
        Into::<Toml>::into(Value::String(String::from("String"))),
    );
    assert_eq!(
        Value::String(String::from("String")),
        Into::<Value>::into(Value::String(String::from("String"))),
    );
}
