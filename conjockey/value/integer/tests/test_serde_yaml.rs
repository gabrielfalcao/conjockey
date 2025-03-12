use crate::integer::Integer;
use crate::value::Value;
use ::serde_yaml::Number as YamlNumber;
use ::serde_yaml::Value as YamlValue;

#[test]
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

// NOTE: In writing tests for `serde_json` and `serde_yaml` the author
// of ConJockey notices that ./test_serde_json.rs has a greater number
// of tests than `file!()` which seems to present the following
// incoherence between the `serde_json` and `serde_yaml`
// implementations:

// - `Into<serde_json::Value>` seems to be implemented `From<Option<T>> where T: From<i64>` which is the case for `conjockey::Integer`. One can check `tests/test_serde_json.rs' for evidence of the present statement
// - `Into<serde_yaml::Value>` seems NOT to be implemented `From<Option<T>> where T: From<i64>`. One can check `tests/test_serde_yaml.rs' for evidence of the present statement, because that test uses the crate `trybuild' to compile `tests/serde_yaml_seeming_incoherence_with_serde_json/yaml_incoherence_caught.rs' expecting a compile error that clearly does not happen in `tests/test_serde_json.rs'
