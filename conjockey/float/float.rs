#[derive(Debug, Copy, Clone)]
pub struct Float(pub f64);

impl Float {
    pub fn as_f64(self) -> f64 {
        self.0
    }
    pub fn as_f32(self) -> f32 {
        self.0 as f32
    }
}

impl PartialEq for Float {
    fn eq(&self, other: &Float) -> bool {
        self.as_f64() == other.as_f64()
    }
}

impl Eq for Float {}
