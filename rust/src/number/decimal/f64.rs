// This is free and unencumbered software released into the public domain.

impl TryFrom<f64> for Decimal {
    type Error = rust_decimal::Error;

    fn try_from(input: f64) -> Result<Self, Self::Error> {
        Ok(Self(rust_decimal::Decimal::try_from(input)?))
    }
}
