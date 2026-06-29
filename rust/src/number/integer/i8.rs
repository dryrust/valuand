// This is free and unencumbered software released into the public domain.

impl From<i8> for Integer {
    fn from(input: i8) -> Self {
        Self::I8(input)
    }
}
