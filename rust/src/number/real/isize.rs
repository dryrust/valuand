// This is free and unencumbered software released into the public domain.

#[cfg(feature = "integer")]
impl From<isize> for Real {
    fn from(input: isize) -> Self {
        Self::Integer(input.into())
    }
}
