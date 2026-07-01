// This is free and unencumbered software released into the public domain.

impl From<f64> for Float {
    fn from(input: f64) -> Self {
        Self::F64(input.into())
    }
}

impl From<Total<f64>> for Float {
    fn from(input: Total<f64>) -> Self {
        Self::F64(input.into())
    }
}
