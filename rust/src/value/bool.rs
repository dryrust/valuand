// This is free and unencumbered software released into the public domain.

impl<T: Debug> From<bool> for Value<T> {
    fn from(input: bool) -> Self {
        Self::Bool(input)
    }
}

impl<T: Debug> From<Value<T>> for bool {
    fn from(input: Value<T>) -> Self {
        From::<&Value<T>>::from(&input)
    }
}

impl<T: Debug> From<&Value<T>> for bool {
    fn from(input: &Value<T>) -> Self {
        match input {
            Value::Bool(value) => *value,
            _ => unreachable!(),
        }
    }
}
