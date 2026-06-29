// This is free and unencumbered software released into the public domain.

#[cfg(feature = "integer")]
impl From<i64> for Decimal {
    fn from(input: i64) -> Self {
        Self(input.into())
    }
}
