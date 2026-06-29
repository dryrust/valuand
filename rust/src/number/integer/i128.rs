// This is free and unencumbered software released into the public domain.

impl From<i128> for Integer {
    fn from(input: i128) -> Self {
        Self::I128(input)
    }
}

impl From<Integer> for i128 {
    fn from(input: Integer) -> Self {
        input.as_i128()
    }
}
