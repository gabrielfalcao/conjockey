mod impls;
#[cfg(test)]
mod tests;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Integer(i64);

impl Integer {
    pub fn as_i64(self) -> i64 {
        self.0
    }
}
