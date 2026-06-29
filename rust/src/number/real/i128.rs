// This is free and unencumbered software released into the public domain.

#[cfg(feature = "integer")]
impl From<i128> for Real {
    fn from(input: i128) -> Self {
        Self::Integer(input.into())
    }
}
