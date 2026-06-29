// This is free and unencumbered software released into the public domain.

impl<T: Debug> From<()> for Value<T> {
    fn from(_: ()) -> Self {
        Self::Unit
    }
}

impl<T: Debug> From<Value<T>> for () {
    fn from(input: Value<T>) -> Self {
        From::<&Value<T>>::from(&input)
    }
}

impl<T: Debug> From<&Value<T>> for () {
    fn from(input: &Value<T>) -> Self {
        match input {
            Value::Unit => (),
            _ => unreachable!(),
        }
    }
}
