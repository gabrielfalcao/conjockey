use crate::value::Value;
use serde_yaml::Value as Yaml;

#[test]
fn test_serde_yaml_string() {
    assert_eq!(
        Yaml::String(String::from("String")),
        Into::<Yaml>::into(Value::String(String::from("String"))),
    );
    assert_eq!(
        Value::String(String::from("String")),
        Into::<Value>::into(Yaml::String(String::from("String"))),
    );
}
