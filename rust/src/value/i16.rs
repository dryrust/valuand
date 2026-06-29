// This is free and unencumbered software released into the public domain.

#[cfg(feature = "integer")]
impl<T: Debug> From<i16> for Value<T> {
    fn from(input: i16) -> Self {
        Self::Number(input.into())
    }
}

#[cfg(feature = "integer")]
impl<T: Debug> From<Value<T>> for i16 {
    fn from(input: Value<T>) -> Self {
        From::<&Value<T>>::from(&input)
    }
}

#[cfg(feature = "integer")]
impl<T: Debug> From<&Value<T>> for i16 {
    fn from(input: &Value<T>) -> Self {
        use super::{Integer, Real};
        match input
            .as_number()
            .expect("i16::from(Value) should be applied to Value::Number only")
        {
            Real::Integer(Integer::I8(z)) => *z as _,
            Real::Integer(Integer::I16(z)) => *z,
            _ => unreachable!(),
        }
    }
}
