// This is free and unencumbered software released into the public domain.

#[cfg(feature = "integer")]
impl<T: Debug> From<u8> for Scalar<T> {
    fn from(input: u8) -> Self {
        Self::Number(input.into())
    }
}

#[cfg(feature = "integer")]
impl<T: Debug> From<Scalar<T>> for u8 {
    fn from(input: Scalar<T>) -> Self {
        From::<&Scalar<T>>::from(&input)
    }
}

#[cfg(feature = "integer")]
impl<T: Debug> From<&Scalar<T>> for u8 {
    fn from(input: &Scalar<T>) -> Self {
        use super::{Natural, Real};
        match input
            .as_number()
            .expect("u8::from(Scalar) should be applied to Scalar::Number only")
        {
            Real::Natural(Natural::U8(n)) => *n,
            _ => unreachable!(),
        }
    }
}
