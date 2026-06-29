// This is free and unencumbered software released into the public domain.

#[cfg(feature = "integer")]
impl From<i32> for Decimal {
    fn from(input: i32) -> Self {
        Self(input.into())
    }
}
