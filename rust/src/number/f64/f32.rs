// This is free and unencumbered software released into the public domain.

impl From<f32> for F64 {
    fn from(input: f32) -> Self {
        Self((input as f64).into())
    }
}

impl From<Total<f32>> for F64 {
    fn from(input: Total<f32>) -> Self {
        Self((input.into_inner() as f64).into())
    }
}

impl From<F64> for f32 {
    fn from(input: F64) -> Self {
        input.0.into_inner() as _
    }
}

impl From<&F64> for f32 {
    fn from(input: &F64) -> Self {
        input.0.into_inner() as _
    }
}
