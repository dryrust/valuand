// This is free and unencumbered software released into the public domain.

impl From<isize> for Integer {
    fn from(input: isize) -> Self {
        Self::I128(input as _)
    }
}

impl From<Integer> for isize {
    fn from(input: Integer) -> Self {
        input.as_i128() as _
    }
}
