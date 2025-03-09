use crate::value::Value;
use serde_json::Value as Json;

#[test]
fn test_serde_json_boolean_false() {
    assert_eq!(
        Json::Bool(false),
        Into::<Json>::into(Value::Boolean(false)),
    );
    assert_eq!(
        Value::Boolean(false),
        Into::<Value>::into(Json::Bool(false)),
    );
}

#[test]
fn test_serde_json_boolean_true() {
    assert_eq!(
        Json::Bool(true),
        Into::<Json>::into(Value::Boolean(true)),
    );
    assert_eq!(
        Value::Boolean(true),
        Into::<Value>::into(Json::Bool(true)),
    );
}
