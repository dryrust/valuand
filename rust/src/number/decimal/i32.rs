// This is free and unencumbered software released into the public domain.

impl From<i32> for Decimal {
    fn from(input: i32) -> Self {
        Self(input.into())
    }
}

impl TryFrom<Decimal> for i32 {
    type Error = ();

    fn try_from(input: Decimal) -> Result<Self, Self::Error> {
        Self::try_from(&input)
    }
}

impl TryFrom<&Decimal> for i32 {
    type Error = ();

    fn try_from(input: &Decimal) -> Result<Self, Self::Error> {
        input.to_i32().ok_or(())
    }
}
