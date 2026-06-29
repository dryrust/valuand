// This is free and unencumbered software released into the public domain.

#[cfg(feature = "float")]
impl<T: Debug> From<f64> for Value<T> {
    fn from(input: f64) -> Self {
        Self::Number(input.into())
    }
}

#[cfg(feature = "float")]
impl<T: Debug> From<Value<T>> for f64 {
    fn from(input: Value<T>) -> Self {
        From::<&Value<T>>::from(&input)
    }
}

#[cfg(feature = "float")]
impl<T: Debug> From<&Value<T>> for f64 {
    fn from(input: &Value<T>) -> Self {
        use super::{Float, Real};
        match input
            .as_number()
            .expect("f64::from(Value) should be applied to Value::Number only")
        {
            Real::Float(Float::F32(r)) => r.into_inner() as _,
            Real::Float(Float::F64(r)) => r.into_inner(),
            _ => unreachable!(),
        }
    }
}
