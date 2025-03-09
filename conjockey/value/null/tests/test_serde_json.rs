use crate::value::Value;
use serde_json::Value as Json;

#[test]
fn test_serde_json_null() {
    assert_eq!(
        Json::Null,
        Into::<Json>::into(Value::Null),
    );
    assert_eq!(
        Value::Null,
        Into::<Value>::into(Json::Null),
    );
}
