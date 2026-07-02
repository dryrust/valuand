// This is free and unencumbered software released into the public domain.

impl<T: Debug> From<()> for Scalar<T> {
    fn from(_: ()) -> Self {
        Self::Unit
    }
}

impl<T: Debug> From<Scalar<T>> for () {
    fn from(input: Scalar<T>) -> Self {
        From::<&Scalar<T>>::from(&input)
    }
}

impl<T: Debug> From<&Scalar<T>> for () {
    fn from(input: &Scalar<T>) -> Self {
        match input {
            Scalar::Unit => (),
            _ => unreachable!(),
        }
    }
}
