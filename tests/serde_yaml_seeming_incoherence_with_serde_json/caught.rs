use conjockey::integer::Integer;
use conjockey::value::Value;
use serde_yaml::Number as YamlNumber;
use serde_yaml::Value as YamlValue;

fn test_yaml_integer() {
    assert_eq!(
        YamlValue::Number(YamlNumber::from(501)),
        Into::<YamlValue>::into(Value::Integer(Integer(501))),
    );
    assert_eq!(
        Value::Integer(Integer(501)),
        Into::<Value>::into(YamlValue::Number(YamlNumber::from(501))),
    );
}

fn test_yaml_integer_from_option_integer() {
    assert_eq!(
        YamlValue::Number(YamlNumber::from(501)),
        Into::<YamlValue>::into(Some(Integer(501))),
    );
}

fn test_yaml_integer_from_option_value_integer() {
    assert_eq!(
        YamlValue::Number(YamlNumber::from(501)),
        Into::<YamlValue>::into(Some(Value::Integer(Integer(501)))),
    );
}

#[test]
fn test() {
    test_yaml_integer();
    test_yaml_integer_from_option_integer();
    test_yaml_integer_from_option_value_integer();
}

fn main() {
    test_yaml_integer();
    test_yaml_integer_from_option_integer();
    test_yaml_integer_from_option_value_integer();
}
