use crate::integer::Integer;

impl std::ops::Add for Integer {
    type Output = Integer;
    fn add(self, rhs: Integer) -> Integer {
        Integer(self.as_i64() + rhs.as_i64())
    }
}

impl std::ops::Sub for Integer {
    type Output = Integer;
    fn sub(self, rhs: Integer) -> Integer {
        Integer(self.as_i64() - rhs.as_i64())
    }
}

impl std::ops::AddAssign for Integer {
    fn add_assign(&mut self, rhs: Integer) {
        self.0 += rhs.as_i64()
    }
}

impl std::ops::SubAssign for Integer {
    fn sub_assign(&mut self, rhs: Integer) {
        self.0 -= rhs.as_i64()
    }
}

impl std::ops::Mul for Integer {
    type Output = Integer;
    fn mul(self, rhs: Integer) -> Integer {
        Integer(self.as_i64() * rhs.as_i64())
    }
}

impl std::ops::Div for Integer {
    type Output = Integer;
    fn div(self, rhs: Integer) -> Integer {
        Integer(self.as_i64() / rhs.as_i64())
    }
}

impl std::ops::MulAssign for Integer {
    fn mul_assign(&mut self, rhs: Integer) {
        self.0 *= rhs.as_i64()
    }
}

impl std::ops::DivAssign for Integer {
    fn div_assign(&mut self, rhs: Integer) {
        self.0 /= rhs.as_i64()
    }
}

impl std::ops::Rem for Integer {
    type Output = Integer;
    fn rem(self, rhs: Integer) -> Integer {
        Integer(self.as_i64() % rhs.as_i64())
    }
}

impl std::ops::RemAssign for Integer {
    fn rem_assign(&mut self, rhs: Integer) {
        self.0 %= rhs.as_i64()
    }
}

impl std::ops::Neg for Integer {
    type Output = Integer;
    fn neg(self) -> Integer {
        Integer(-self.as_i64())
    }
}

impl std::ops::Not for Integer {
    type Output = Integer;
    fn not(self) -> Integer {
        Integer(!self.as_i64())
    }
}

impl std::cmp::PartialOrd for Integer {
    fn partial_cmp(&self, rhs: &Integer) -> Option<std::cmp::Ordering> {
        self.as_i64().partial_cmp(&rhs.as_i64())
    }
}

impl std::cmp::Ord for Integer {
    fn cmp(&self, rhs: &Integer) -> std::cmp::Ordering {
        self.as_i64().cmp(&rhs.as_i64())
    }
}

impl crate::traits::ops::algebraic::AlgebraicOperations for Integer {}
