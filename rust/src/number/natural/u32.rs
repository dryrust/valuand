// This is free and unencumbered software released into the public domain.

impl From<u32> for Natural {
    fn from(input: u32) -> Self {
        Self::U32(input)
    }
}
