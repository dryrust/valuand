// This is free and unencumbered software released into the public domain.

#[cfg(feature = "integer")]
impl<T: Debug> From<u16> for Value<T> {
    fn from(input: u16) -> Self {
        Self::Number(input.into())
    }
}

#[cfg(feature = "integer")]
impl<T: Debug> From<Value<T>> for u16 {
    fn from(input: Value<T>) -> Self {
        From::<&Value<T>>::from(&input)
    }
}

#[cfg(feature = "integer")]
impl<T: Debug> From<&Value<T>> for u16 {
    fn from(input: &Value<T>) -> Self {
        use super::{Natural, Real};
        match input
            .as_number()
            .expect("u16::from(Value) should be applied to Value::Number only")
        {
            Real::Natural(Natural::U8(n)) => *n as _,
            Real::Natural(Natural::U16(n)) => *n,
            _ => unreachable!(),
        }
    }
}
