// This is free and unencumbered software released into the public domain.

impl From<i32> for Integer {
    fn from(input: i32) -> Self {
        Self::I32(input)
    }
}
