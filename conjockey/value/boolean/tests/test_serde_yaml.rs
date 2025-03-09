use crate::value::Value;
use serde_yaml::Value as Yaml;

#[test]
fn test_serde_yaml_boolean_false() {
    assert_eq!(
        Yaml::Bool(false),
        Into::<Yaml>::into(Value::Boolean(false)),
    );
    assert_eq!(
        Value::Boolean(false),
        Into::<Value>::into(Yaml::Bool(false)),
    );
}

#[test]
fn test_serde_yaml_boolean_true() {
    assert_eq!(
        Yaml::Bool(true),
        Into::<Yaml>::into(Value::Boolean(true)),
    );
    assert_eq!(
        Value::Boolean(true),
        Into::<Value>::into(Yaml::Bool(true)),
    );
}
