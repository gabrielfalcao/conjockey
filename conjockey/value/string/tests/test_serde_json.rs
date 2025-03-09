use crate::value::Value;
use serde_json::Value as Json;

#[test]
fn test_serde_json_string() {
    assert_eq!(
        Json::String(String::from("String")),
        Into::<Json>::into(Value::String(String::from("String"))),
    );
    assert_eq!(
        Value::String(String::from("String")),
        Into::<Value>::into(Value::String(String::from("String"))),
    );
}
