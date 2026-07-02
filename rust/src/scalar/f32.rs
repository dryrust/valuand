// This is free and unencumbered software released into the public domain.

#[cfg(feature = "float")]
impl<T: Debug> From<f32> for Scalar<T> {
    fn from(input: f32) -> Self {
        Self::Number(input.into())
    }
}

#[cfg(feature = "float")]
impl<T: Debug> From<Scalar<T>> for f32 {
    fn from(input: Scalar<T>) -> Self {
        From::<&Scalar<T>>::from(&input)
    }
}

#[cfg(feature = "float")]
impl<T: Debug> From<&Scalar<T>> for f32 {
    fn from(input: &Scalar<T>) -> Self {
        use super::{Float, Real};
        match input
            .as_number()
            .expect("f32::from(Scalar) should be applied to Scalar::Number only")
        {
            Real::Float(Float::F32(r)) => r.into_inner(),
            _ => unreachable!(),
        }
    }
}
