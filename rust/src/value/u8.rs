// This is free and unencumbered software released into the public domain.

#[cfg(feature = "integer")]
impl<T: Debug> From<u8> for Value<T> {
    fn from(input: u8) -> Self {
        Self::Number(input.into())
    }
}

#[cfg(feature = "integer")]
impl<T: Debug> From<Value<T>> for u8 {
    fn from(input: Value<T>) -> Self {
        From::<&Value<T>>::from(&input)
    }
}

#[cfg(feature = "integer")]
impl<T: Debug> From<&Value<T>> for u8 {
    fn from(input: &Value<T>) -> Self {
        use super::{Natural, Real};
        match input
            .as_number()
            .expect("u8::from(Value) should be applied to Value::Number only")
        {
            Real::Natural(Natural::U8(n)) => *n,
            _ => unreachable!(),
        }
    }
}
