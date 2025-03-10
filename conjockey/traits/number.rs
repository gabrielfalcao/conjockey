use crate::errors::Error;
use crate::traits::ops::{AlgebraicOperations, LogicalOperations};

pub trait Number:
    Sized
    + Copy
    + Clone
    + std::fmt::Binary
    + std::fmt::Octal
    + std::fmt::Display
    + std::default::Default
    + std::marker::Send
    + std::marker::Sync
    + AlgebraicOperations
    + LogicalOperations
{
}

pub trait NumberFromString: Number {
    fn from_string(value: impl std::fmt::Display) -> Result<Self, Error> {
        let value = value.to_string();
        Self::from_str(&value)
    }
    fn from_str(value: &str) -> Result<Self, Error>;
}
