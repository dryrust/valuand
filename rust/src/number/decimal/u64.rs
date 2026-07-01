// This is free and unencumbered software released into the public domain.

impl From<u64> for Decimal {
    fn from(input: u64) -> Self {
        Self(input.into())
    }
}

impl TryFrom<Decimal> for u64 {
    type Error = ();

    fn try_from(input: Decimal) -> Result<Self, Self::Error> {
        Self::try_from(&input)
    }
}

impl TryFrom<&Decimal> for u64 {
    type Error = ();

    fn try_from(input: &Decimal) -> Result<Self, Self::Error> {
        input.to_u64().ok_or(())
    }
}
