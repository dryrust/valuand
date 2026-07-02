// This is free and unencumbered software released into the public domain.

#[cfg(feature = "integer")]
impl<T: Debug> From<i128> for Scalar<T> {
    fn from(input: i128) -> Self {
        Self::Number(input.into())
    }
}

#[cfg(feature = "integer")]
impl<T: Debug> From<Scalar<T>> for i128 {
    fn from(input: Scalar<T>) -> Self {
        From::<&Scalar<T>>::from(&input)
    }
}

#[cfg(feature = "integer")]
impl<T: Debug> From<&Scalar<T>> for i128 {
    fn from(input: &Scalar<T>) -> Self {
        use super::{Integer, Real};
        match input
            .as_number()
            .expect("i128::from(Scalar) should be applied to Scalar::Number only")
        {
            Real::Integer(Integer::I8(z)) => *z as _,
            Real::Integer(Integer::I16(z)) => *z as _,
            Real::Integer(Integer::I32(z)) => *z as _,
            Real::Integer(Integer::I64(z)) => *z as _,
            Real::Integer(Integer::I128(z)) => *z,
            _ => unreachable!(),
        }
    }
}
