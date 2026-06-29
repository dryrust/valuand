// This is free and unencumbered software released into the public domain.

impl From<u128> for Natural {
    fn from(input: u128) -> Self {
        Self::U128(input)
    }
}

impl From<Natural> for u128 {
    fn from(input: Natural) -> Self {
        input.as_u128()
    }
}
