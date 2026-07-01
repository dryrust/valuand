// This is free and unencumbered software released into the public domain.

impl TryFrom<f32> for Decimal {
    type Error = rust_decimal::Error;

    fn try_from(input: f32) -> Result<Self, Self::Error> {
        Ok(Self(rust_decimal::Decimal::try_from(input)?))
    }
}

impl TryFrom<Decimal> for f32 {
    type Error = ();

    fn try_from(input: Decimal) -> Result<Self, Self::Error> {
        Self::try_from(&input)
    }
}

impl TryFrom<&Decimal> for f32 {
    type Error = ();

    fn try_from(input: &Decimal) -> Result<Self, Self::Error> {
        input.to_f32().ok_or(())
    }
}
