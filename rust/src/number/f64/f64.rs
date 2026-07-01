// This is free and unencumbered software released into the public domain.

impl From<f64> for F64 {
    fn from(input: f64) -> Self {
        Self(input.into())
    }
}

impl From<Total<f64>> for F64 {
    fn from(input: Total<f64>) -> Self {
        Self(input.into())
    }
}

impl From<F64> for f64 {
    fn from(input: F64) -> Self {
        input.0.into_inner() as _
    }
}

impl From<&F64> for f64 {
    fn from(input: &F64) -> Self {
        input.0.into_inner() as _
    }
}
