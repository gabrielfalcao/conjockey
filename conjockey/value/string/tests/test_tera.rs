use crate::value::Value;
use tera::Value as Tera;

#[test]
fn test_serde_tera_string() {
    assert_eq!(
        Tera::String(String::from("String")),
        Into::<Tera>::into(Value::String(String::from("String"))),
    );
    assert_eq!(
        Value::String(String::from("String")),
        Into::<Value>::into(Tera::String(String::from("String"))),
    );
}
