// This is free and unencumbered software released into the public domain.

#[cfg(feature = "integer")]
impl From<i32> for Real {
    fn from(input: i32) -> Self {
        Self::Integer(input.into())
    }
}
