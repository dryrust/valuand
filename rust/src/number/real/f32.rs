// This is free and unencumbered software released into the public domain.

#[cfg(feature = "float")]
impl From<f32> for Real {
    fn from(input: f32) -> Self {
        Self::Float(input.into())
    }
}

#[cfg(feature = "float")]
impl From<decorum::Total<f32>> for Real {
    fn from(input: decorum::Total<f32>) -> Self {
        Self::Float(input.into())
    }
}
