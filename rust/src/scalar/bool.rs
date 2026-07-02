// This is free and unencumbered software released into the public domain.

impl<T: Debug> From<bool> for Scalar<T> {
    fn from(input: bool) -> Self {
        Self::Bool(input)
    }
}

impl<T: Debug> From<Scalar<T>> for bool {
    fn from(input: Scalar<T>) -> Self {
        From::<&Scalar<T>>::from(&input)
    }
}

impl<T: Debug> From<&Scalar<T>> for bool {
    fn from(input: &Scalar<T>) -> Self {
        match input {
            Scalar::Bool(value) => *value,
            _ => unreachable!(),
        }
    }
}
