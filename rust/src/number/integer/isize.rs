// This is free and unencumbered software released into the public domain.

impl From<isize> for Integer {
    fn from(input: isize) -> Self {
        Self::I128(input as _)
    }
}

impl TryFrom<Integer> for isize {
    type Error = ();

    fn try_from(input: Integer) -> Result<Self, Self::Error> {
        Self::try_from(&input)
    }
}

impl TryFrom<&Integer> for isize {
    type Error = ();

    fn try_from(input: &Integer) -> Result<Self, Self::Error> {
        input.to_i128().ok_or(()).map(|z| z as _)
    }
}
