// This is free and unencumbered software released into the public domain.

impl From<u8> for Natural {
    fn from(input: u8) -> Self {
        Self::U8(input)
    }
}

impl TryFrom<Natural> for u8 {
    type Error = ();

    fn try_from(input: Natural) -> Result<Self, Self::Error> {
        Self::try_from(&input)
    }
}

impl TryFrom<&Natural> for u8 {
    type Error = ();

    fn try_from(input: &Natural) -> Result<Self, Self::Error> {
        input.to_u8().ok_or(())
    }
}
