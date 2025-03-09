use crate::value::Value;


#[test]
fn test_value_string_string() {
    assert_eq!(Into::<Value>::into(String::from("String")), Value::String(String::from("String")));
}


#[test]
fn test_value_string_str() {
    assert_eq!(Into::<Value>::into("str"), Value::String(String::from("str")));
}
