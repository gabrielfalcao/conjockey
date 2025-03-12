use crate::value::Value;

#[test]
fn test_value_string_string() {
    assert_eq!(
        Into::<Value>::into(String::from("String")),
        Value::String(String::from("String"))
    );
}

#[test]
fn test_value_string_str() {
    assert_eq!(
        Into::<Value>::into("str"),
        Value::String(String::from("str"))
    );
}

#[test]
fn test_option_string_into_value_string() {
    assert_eq!(
        Into::<Value>::into(Some(String::from("String"))),
        Value::String(String::from("String"))
    );
}

#[test]
fn test_option_str_into_value_string() {
    assert_eq!(
        Into::<Value>::into(Some("String")),
        Value::String(String::from("String"))
    );
}
