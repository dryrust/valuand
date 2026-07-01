// This is free and unencumbered software released into the public domain.

impl From<isize> for Decimal {
    fn from(input: isize) -> Self {
        Self(input.into())
    }
}

impl TryFrom<Decimal> for isize {
    type Error = ();

    fn try_from(input: Decimal) -> Result<Self, Self::Error> {
        Self::try_from(&input)
    }
}

impl TryFrom<&Decimal> for isize {
    type Error = ();

    fn try_from(input: &Decimal) -> Result<Self, Self::Error> {
        input.to_isize().ok_or(())
    }
}
