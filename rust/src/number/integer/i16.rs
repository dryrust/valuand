// This is free and unencumbered software released into the public domain.

impl From<i16> for Integer {
    fn from(input: i16) -> Self {
        Self::I16(input)
    }
}
