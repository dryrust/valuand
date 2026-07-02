// This is free and unencumbered software released into the public domain.

#[cfg(feature = "integer")]
impl<T: Debug> From<u64> for Scalar<T> {
    fn from(input: u64) -> Self {
        Self::Number(input.into())
    }
}

#[cfg(feature = "integer")]
impl<T: Debug> From<Scalar<T>> for u64 {
    fn from(input: Scalar<T>) -> Self {
        From::<&Scalar<T>>::from(&input)
    }
}

#[cfg(feature = "integer")]
impl<T: Debug> From<&Scalar<T>> for u64 {
    fn from(input: &Scalar<T>) -> Self {
        use super::{Natural, Real};
        match input
            .as_number()
            .expect("u64::from(Scalar) should be applied to Scalar::Number only")
        {
            Real::Natural(Natural::U8(n)) => *n as _,
            Real::Natural(Natural::U16(n)) => *n as _,
            Real::Natural(Natural::U32(n)) => *n as _,
            Real::Natural(Natural::U64(n)) => *n,
            _ => unreachable!(),
        }
    }
}
