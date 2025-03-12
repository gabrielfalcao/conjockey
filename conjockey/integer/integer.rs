#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Integer(pub i64);

impl Integer {
    pub fn as_i64(self) -> i64 {
        self.0
    }
}
