use conjockey::integer::Integer;
use conjockey::value::Value;
use serde_json::Number as JsonNumber;
use serde_json::Value as JsonValue;

fn test_json_integer() {
    assert_eq!(
        JsonValue::Number(JsonNumber::from(501)),
        Into::<JsonValue>::into(Value::Integer(Integer(501))),
    );
    assert_eq!(
        Value::Integer(Integer(501)),
        Into::<Value>::into(JsonValue::Number(JsonNumber::from(501))),
    );
}

fn test_json_integer_from_option_integer() {
    assert_eq!(
        JsonValue::Number(JsonNumber::from(501)),
        Into::<JsonValue>::into(Some(Integer(501))),
    );
}

fn test_json_integer_from_option_value_integer() {
    assert_eq!(
        JsonValue::Number(JsonNumber::from(501)),
        Into::<JsonValue>::into(Some(Value::Integer(Integer(501)))),
    );
}

#[test]
fn test() {
    test_json_integer();
    test_json_integer_from_option_integer();
    test_json_integer_from_option_value_integer();
}

fn main() {
    test_json_integer();
    test_json_integer_from_option_integer();
    test_json_integer_from_option_value_integer();
}
