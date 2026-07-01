// This is free and unencumbered software released into the public domain.

impl From<u64> for Natural {
    fn from(input: u64) -> Self {
        Self::U64(input)
    }
}

impl TryFrom<Natural> for u64 {
    type Error = ();

    fn try_from(input: Natural) -> Result<Self, Self::Error> {
        Self::try_from(&input)
    }
}

impl TryFrom<&Natural> for u64 {
    type Error = ();

    fn try_from(input: &Natural) -> Result<Self, Self::Error> {
        input.to_u64().ok_or(())
    }
}
