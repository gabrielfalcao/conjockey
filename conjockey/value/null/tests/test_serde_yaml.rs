use crate::value::Value;
use serde_yaml::Value as Yaml;

#[test]
fn test_serde_yaml_null() {
    assert_eq!(
        Yaml::Null,
        Into::<Yaml>::into(Value::Null),
    );
    assert_eq!(
        Value::Null,
        Into::<Value>::into(Yaml::Null),
    );
}
