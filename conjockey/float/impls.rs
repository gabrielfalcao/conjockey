use crate::errors::Error;
use crate::float::Float;
use crate::value::Value;
use std::any::TypeId;

impl std::ops::Add for Float {
    type Output = Float;
    fn add(self, rhs: Float) -> Float {
        Float(self.as_f64() + rhs.as_f64())
    }
}

impl std::ops::Sub for Float {
    type Output = Float;
    fn sub(self, rhs: Float) -> Float {
        Float(self.as_f64() - rhs.as_f64())
    }
}

impl std::ops::AddAssign for Float {
    fn add_assign(&mut self, rhs: Float) {
        self.0 += rhs.as_f64()
    }
}

impl std::ops::SubAssign for Float {
    fn sub_assign(&mut self, rhs: Float) {
        self.0 -= rhs.as_f64()
    }
}

impl std::ops::Mul for Float {
    type Output = Float;
    fn mul(self, rhs: Float) -> Float {
        Float(self.as_f64() * rhs.as_f64())
    }
}

impl std::ops::Div for Float {
    type Output = Float;
    fn div(self, rhs: Float) -> Float {
        Float(self.as_f64() / rhs.as_f64())
    }
}

impl std::ops::MulAssign for Float {
    fn mul_assign(&mut self, rhs: Float) {
        self.0 *= rhs.as_f64()
    }
}

impl std::ops::DivAssign for Float {
    fn div_assign(&mut self, rhs: Float) {
        self.0 /= rhs.as_f64()
    }
}

impl std::ops::Rem for Float {
    type Output = Float;
    fn rem(self, rhs: Float) -> Float {
        Float(self.as_f64() % rhs.as_f64())
    }
}

impl std::ops::RemAssign for Float {
    fn rem_assign(&mut self, rhs: Float) {
        self.0 %= rhs.as_f64()
    }
}

impl std::ops::Neg for Float {
    type Output = Float;
    fn neg(self) -> Float {
        Float(-self.as_f64())
    }
}

impl std::cmp::PartialOrd for Float {
    fn partial_cmp(&self, rhs: &Float) -> Option<std::cmp::Ordering> {
        self.as_f64().partial_cmp(&rhs.as_f64())
    }
}

impl std::cmp::Ord for Float {
    fn cmp(&self, rhs: &Float) -> std::cmp::Ordering {
        if self.as_f64() < rhs.as_f64() {
            std::cmp::Ordering::Less
        } else if self.as_f64() > rhs.as_f64() {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    }
}

impl From<f64> for Float {
    fn from(value: f64) -> Float {
        Float(value)
    }
}

impl From<f32> for Float {
    fn from(value: f32) -> Float {
        Float(Into::<f64>::into(value))
    }
}

impl Float {
    pub fn is_signed(self) -> bool {
        self.0 < 0.0
    }
    pub fn is_unsigned(self) -> bool {
        self.0 >= 0.0
    }
    pub fn as_signed(self) -> Option<f64> {
        self.as_signed_f64()
    }
    pub fn as_signed_f64(self) -> Option<f64> {
        match TryInto::<f64>::try_into(self.0) {
            Ok(value) => Some(value),
            Err(_) => None,
        }
    }
    pub fn to_signed(self) -> f64 {
        self.to_signed_f64()
    }
    pub fn to_signed_f64(self) -> f64 {
        match self.as_signed() {
            Some(value) => value,
            None => panic!("{} cannot be converted into f64", self.0),
        }
    }
}

impl Into<f64> for Float {
    fn into(self) -> f64 {
        self.to_signed_f64()
    }
}

impl TryFrom<Value> for Float {
    type Error = Error;
    fn try_from(value: Value) -> Result<Float, Error> {
        match value {
            Value::Float(value) => Ok(value),
            _ => Err(Error::TryFromError(format!(
                "can not convert {:#?} to {:#?}",
                value,
                TypeId::of::<Float>()
            ))),
        }
    }
}
