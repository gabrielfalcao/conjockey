use crate::float::Float;
use crate::value::Value;
use ::serde_yaml::Number as YamlNumber;
use ::serde_yaml::Value as YamlValue;

#[test]
fn test_yaml_float() {
    assert_eq!(
        YamlValue::Number(YamlNumber::from(2.718281828459045f64)),
        Into::<YamlValue>::into(Value::Float(Float(2.718281828459045f64))),
    );
    assert_eq!(
        Value::Float(Float(2.718281828459045f64)),
        Into::<Value>::into(YamlValue::Number(YamlNumber::from(2.718281828459045f64))),
    );
}
