// This is free and unencumbered software released into the public domain.

#[cfg(feature = "integer")]
impl From<u8> for Real {
    fn from(input: u8) -> Self {
        Self::Natural(input.into())
    }
}
