// This is free and unencumbered software released into the public domain.

impl From<f32> for Float {
    fn from(input: f32) -> Self {
        Self::F32(input.into())
    }
}

impl From<Total<f32>> for Float {
    fn from(input: Total<f32>) -> Self {
        Self::F32(input.into())
    }
}
