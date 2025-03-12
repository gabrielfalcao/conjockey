use crate::integer::Integer;
use crate::value::Value;

#[test]
fn test_integer_into_value_integer() {
    assert_eq!(Into::<Value>::into(901i64), Value::Integer(Integer(901)));
    assert_eq!(Into::<Value>::into(2502i32), Value::Integer(Integer(2502)));
    assert_eq!(Into::<Value>::into(250i16), Value::Integer(Integer(250)));
    assert_eq!(Into::<Value>::into(25i8), Value::Integer(Integer(25)));
}


#[test]
fn test_value_integer_into_integer() {
    assert_eq!(Value::Integer(Integer(901)), Into::<Value>::into(901i64));
    assert_eq!(Value::Integer(Integer(2502)), Into::<Value>::into(2502i32));
    assert_eq!(Value::Integer(Integer(250)), Into::<Value>::into(250i16));
    assert_eq!(Value::Integer(Integer(25)), Into::<Value>::into(25i8));
}


#[test]
fn test_option_integer_into_value_integer() {
    assert_eq!(Into::<Value>::into(Some(1000i64)), Value::Integer(Integer(1000)));
    assert_eq!(Into::<Value>::into(Some(1000i32)), Value::Integer(Integer(1000)));
    assert_eq!(Into::<Value>::into(Some(1000i16)), Value::Integer(Integer(1000)));
    assert_eq!(Into::<Value>::into(Some(81i8)), Value::Integer(Integer(81)));
}
