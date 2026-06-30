// This is free and unencumbered software released into the public domain.

impl From<usize> for Natural {
    fn from(input: usize) -> Self {
        Self::U128(input as _)
    }
}

impl From<Natural> for usize {
    fn from(input: Natural) -> Self {
        input.as_u128() as _
    }
}
