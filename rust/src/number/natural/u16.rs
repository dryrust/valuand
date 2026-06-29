// This is free and unencumbered software released into the public domain.

impl From<u16> for Natural {
    fn from(input: u16) -> Self {
        Self::U16(input)
    }
}
