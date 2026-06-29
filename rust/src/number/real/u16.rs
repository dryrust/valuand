// This is free and unencumbered software released into the public domain.

#[cfg(feature = "integer")]
impl From<u16> for Real {
    fn from(input: u16) -> Self {
        Self::Natural(input.into())
    }
}
