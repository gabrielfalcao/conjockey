use crate::integer::Integer;
use crate::value::Value;
use toml::Value as Toml;

#[test]
fn test_toml_integer() {
    assert_eq!(
        Toml::Integer(501),
        Into::<Toml>::into(Value::Integer(Integer(501))),
    );
    assert_eq!(
        Value::Integer(Integer(501)),
        Into::<Value>::into(Toml::Integer(501)),
    );
}

#[test]
fn test_toml_integer_from_option() {
    assert_eq!(
        Toml::Integer(501),
        Into::<Toml>::into(Value::from(Some(501)))
    );
}
