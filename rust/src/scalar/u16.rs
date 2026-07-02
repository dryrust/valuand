// This is free and unencumbered software released into the public domain.

#[cfg(feature = "integer")]
impl<T: Debug> From<u16> for Scalar<T> {
    fn from(input: u16) -> Self {
        Self::Number(input.into())
    }
}

#[cfg(feature = "integer")]
impl<T: Debug> From<Scalar<T>> for u16 {
    fn from(input: Scalar<T>) -> Self {
        From::<&Scalar<T>>::from(&input)
    }
}

#[cfg(feature = "integer")]
impl<T: Debug> From<&Scalar<T>> for u16 {
    fn from(input: &Scalar<T>) -> Self {
        use super::{Natural, Real};
        match input
            .as_number()
            .expect("u16::from(Scalar) should be applied to Scalar::Number only")
        {
            Real::Natural(Natural::U8(n)) => *n as _,
            Real::Natural(Natural::U16(n)) => *n,
            _ => unreachable!(),
        }
    }
}
