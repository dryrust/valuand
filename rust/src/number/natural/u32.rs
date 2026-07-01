// This is free and unencumbered software released into the public domain.

impl From<u32> for Natural {
    fn from(input: u32) -> Self {
        Self::U32(input)
    }
}

impl TryFrom<Natural> for u32 {
    type Error = ();

    fn try_from(input: Natural) -> Result<Self, Self::Error> {
        Self::try_from(&input)
    }
}

impl TryFrom<&Natural> for u32 {
    type Error = ();

    fn try_from(input: &Natural) -> Result<Self, Self::Error> {
        input.to_u32().ok_or(())
    }
}
