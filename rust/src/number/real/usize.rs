// This is free and unencumbered software released into the public domain.

#[cfg(feature = "integer")]
impl From<usize> for Real {
    fn from(input: usize) -> Self {
        Self::Natural(input.into())
    }
}
