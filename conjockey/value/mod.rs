mod boolean;
mod json;
mod string;
mod toml;
mod yaml;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Value {
    Boolean(bool),
    String(String),
    // Integer(Integer),
    // Float(Float),
    // DateTime(DateTime),
    // Array(Array),
    // Map(Map<String, Value>),
}
