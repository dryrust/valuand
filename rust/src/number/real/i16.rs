// This is free and unencumbered software released into the public domain.

#[cfg(feature = "integer")]
impl From<i16> for Real {
    fn from(input: i16) -> Self {
        Self::Integer(input.into())
    }
}
