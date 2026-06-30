// This is free and unencumbered software released into the public domain.

impl TryFrom<f32> for Decimal {
    type Error = rust_decimal::Error;

    fn try_from(input: f32) -> Result<Self, Self::Error> {
        Ok(Self(rust_decimal::Decimal::try_from(input)?))
    }
}
