// This is free and unencumbered software released into the public domain.

impl From<u16> for Natural {
    fn from(input: u16) -> Self {
        Self::U16(input)
    }
}

impl TryFrom<Natural> for u16 {
    type Error = ();

    fn try_from(input: Natural) -> Result<Self, Self::Error> {
        Self::try_from(&input)
    }
}

impl TryFrom<&Natural> for u16 {
    type Error = ();

    fn try_from(input: &Natural) -> Result<Self, Self::Error> {
        input.to_u16().ok_or(())
    }
}
