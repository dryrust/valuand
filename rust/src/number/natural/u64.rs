// This is free and unencumbered software released into the public domain.

impl From<u64> for Natural {
    fn from(input: u64) -> Self {
        Self::U64(input)
    }
}
