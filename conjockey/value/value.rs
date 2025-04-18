use crate::integer::Integer;
use crate::float::Float;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Value {
    Null,
    Boolean(bool),
    String(String),
    Integer(Integer),
    Float(Float),
    // DateTime(DateTime),
    // Array(Array),
    // Map(Map<String, Value>),
    Unknown,
}
