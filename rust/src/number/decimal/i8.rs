// This is free and unencumbered software released into the public domain.

impl From<i8> for Decimal {
    fn from(input: i8) -> Self {
        Self(input.into())
    }
}

impl TryFrom<Decimal> for i8 {
    type Error = ();

    fn try_from(input: Decimal) -> Result<Self, Self::Error> {
        Self::try_from(&input)
    }
}

impl TryFrom<&Decimal> for i8 {
    type Error = ();

    fn try_from(input: &Decimal) -> Result<Self, Self::Error> {
        input.to_i8().ok_or(())
    }
}
