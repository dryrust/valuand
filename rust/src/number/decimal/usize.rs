// This is free and unencumbered software released into the public domain.

impl From<usize> for Decimal {
    fn from(input: usize) -> Self {
        Self(input.into())
    }
}

impl TryFrom<Decimal> for usize {
    type Error = ();

    fn try_from(input: Decimal) -> Result<Self, Self::Error> {
        Self::try_from(&input)
    }
}

impl TryFrom<&Decimal> for usize {
    type Error = ();

    fn try_from(input: &Decimal) -> Result<Self, Self::Error> {
        input.to_usize().ok_or(())
    }
}
