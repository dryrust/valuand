// This is free and unencumbered software released into the public domain.

impl TryFrom<f64> for Decimal {
    type Error = crate::DecimalError;

    fn try_from(input: f64) -> Result<Self, Self::Error> {
        Ok(Self(
            rust_decimal::Decimal::try_from(input).map_err(crate::DecimalError::from)?,
        ))
    }
}

impl TryFrom<Decimal> for f64 {
    type Error = ();

    fn try_from(input: Decimal) -> Result<Self, Self::Error> {
        Self::try_from(&input)
    }
}

impl TryFrom<&Decimal> for f64 {
    type Error = ();

    fn try_from(input: &Decimal) -> Result<Self, Self::Error> {
        input.to_f64().ok_or(())
    }
}
