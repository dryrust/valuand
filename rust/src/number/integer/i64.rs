// This is free and unencumbered software released into the public domain.

impl From<i64> for Integer {
    fn from(input: i64) -> Self {
        Self::I64(input)
    }
}
