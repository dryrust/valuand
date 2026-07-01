// This is free and unencumbered software released into the public domain.

impl From<usize> for Natural {
    fn from(input: usize) -> Self {
        Self::U128(input as _)
    }
}

impl TryFrom<Natural> for usize {
    type Error = ();

    fn try_from(input: Natural) -> Result<Self, Self::Error> {
        Self::try_from(&input)
    }
}

impl TryFrom<&Natural> for usize {
    type Error = ();

    fn try_from(input: &Natural) -> Result<Self, Self::Error> {
        input.to_u128().ok_or(()).map(|n| n as _)
    }
}
