// This is free and unencumbered software released into the public domain.

#[cfg(feature = "float")]
impl<T: Debug> From<f32> for Value<T> {
    fn from(input: f32) -> Self {
        Self::Number(input.into())
    }
}

#[cfg(feature = "float")]
impl<T: Debug> From<Value<T>> for f32 {
    fn from(input: Value<T>) -> Self {
        From::<&Value<T>>::from(&input)
    }
}

#[cfg(feature = "float")]
impl<T: Debug> From<&Value<T>> for f32 {
    fn from(input: &Value<T>) -> Self {
        use super::{Float, Real};
        match input
            .as_number()
            .expect("f32::from(Value) should be applied to Value::Number only")
        {
            Real::Float(Float::F32(r)) => r.into_inner(),
            _ => unreachable!(),
        }
    }
}
