// This is free and unencumbered software released into the public domain.

impl From<u8> for Decimal {
    fn from(input: u8) -> Self {
        Self(input.into())
    }
}

impl TryFrom<Decimal> for u8 {
    type Error = ();

    fn try_from(input: Decimal) -> Result<Self, Self::Error> {
        Self::try_from(&input)
    }
}

impl TryFrom<&Decimal> for u8 {
    type Error = ();

    fn try_from(input: &Decimal) -> Result<Self, Self::Error> {
        input.to_u8().ok_or(())
    }
}
