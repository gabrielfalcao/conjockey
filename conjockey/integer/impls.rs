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

impl From<i64> for Integer {
    fn from(value: i64) -> Integer {
        Integer(value)
    }
}

impl From<i32> for Integer {
    fn from(value: i32) -> Integer {
        Integer(Into::<i64>::into(value))
    }
}

impl Integer {
    pub fn is_signed(self) -> bool {
        self.0 < 0
    }
    pub fn is_unsigned(self) -> bool {
        self.0 >= 0
    }
    pub fn as_signed(self) -> Option<i64> {
        if self.is_signed() {
            Some(self.0)
        } else {
            None
        }
    }
    pub fn as_unsigned(self) -> Option<u64> {
        self.as_unsigned_u64()
    }
    pub fn as_unsigned_u64(self) -> Option<u64> {
        match TryInto::<u64>::try_into(self.0) {
            Ok(value) => Some(value),
            Err(_) => None,
        }
    }
    pub fn as_unsigned_u32(self) -> Option<u32> {
        match TryInto::<u32>::try_into(self.0) {
            Ok(value) => Some(value),
            Err(_) => None,
        }
    }
    pub fn as_unsigned_u16(self) -> Option<u16> {
        match TryInto::<u16>::try_into(self.0) {
            Ok(value) => Some(value),
            Err(_) => None,
        }
    }
    pub fn as_unsigned_u8(self) -> Option<u8> {
        match TryInto::<u8>::try_into(self.0) {
            Ok(value) => Some(value),
            Err(_) => None,
        }
    }
    pub fn to_unsigned(self) -> u64 {
        self.to_unsigned_u64()
    }
    pub fn to_unsigned_u64(self) -> u64 {
        match self.as_unsigned_u64() {
            Some(value) => value,
            None => panic!("{} cannot be converted into u64", self.0),
        }
    }
    pub fn to_unsigned_u32(self) -> u32 {
        match self.as_unsigned_u32() {
            Some(value) => value,
            None => panic!("{} cannot be converted into u32", self.0),
        }
    }
    pub fn to_unsigned_u16(self) -> u16 {
        match self.as_unsigned_u16() {
            Some(value) => value,
            None => panic!("{} cannot be converted into u16", self.0),
        }
    }
    pub fn to_unsigned_u8(self) -> u8 {
        match self.as_unsigned_u8() {
            Some(value) => value,
            None => panic!("{} cannot be converted into u8", self.0),
        }
    }
}

impl std::ops::BitAnd for Integer {
    type Output = Integer;
    fn bitand(self, rhs: Integer) -> Integer {
        Integer(self.as_i64() & rhs.as_i64())
    }
}

impl std::ops::BitOr for Integer {
    type Output = Integer;
    fn bitor(self, rhs: Integer) -> Integer {
        Integer(self.as_i64() | rhs.as_i64())
    }
}

impl std::ops::BitAndAssign for Integer {
    fn bitand_assign(&mut self, rhs: Integer) {
        self.0 &= rhs.as_i64()
    }
}

impl std::ops::BitOrAssign for Integer {
    fn bitor_assign(&mut self, rhs: Integer) {
        self.0 |= rhs.as_i64()
    }
}

impl std::ops::BitXor for Integer {
    type Output = Integer;
    fn bitxor(self, rhs: Integer) -> Integer {
        Integer(self.as_i64() ^ rhs.as_i64())
    }
}

impl std::ops::BitXorAssign for Integer {
    fn bitxor_assign(&mut self, rhs: Integer) {
        self.0 ^= rhs.as_i64()
    }
}

impl std::ops::Shr for Integer {
    type Output = Integer;
    fn shr(self, rhs: Integer) -> Integer {
        Integer(self.as_i64() >> rhs.as_i64())
    }
}

impl std::ops::Shl for Integer {
    type Output = Integer;
    fn shl(self, rhs: Integer) -> Integer {
        Integer(self.as_i64() << rhs.as_i64())
    }
}

impl std::ops::ShrAssign for Integer {
    fn shr_assign(&mut self, rhs: Integer) {
        self.0 >>= rhs.as_i64()
    }
}

impl std::ops::ShlAssign for Integer {
    fn shl_assign(&mut self, rhs: Integer) {
        self.0 <<= rhs.as_i64()
    }
}

impl crate::traits::ops::LogicalOperations for Integer {}
