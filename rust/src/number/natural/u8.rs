// This is free and unencumbered software released into the public domain.

impl From<u8> for Natural {
    fn from(input: u8) -> Self {
        Self::U8(input)
    }
}
