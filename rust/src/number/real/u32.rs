// This is free and unencumbered software released into the public domain.

#[cfg(feature = "integer")]
impl From<u32> for Real {
    fn from(input: u32) -> Self {
        Self::Natural(input.into())
    }
}
