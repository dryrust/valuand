// This is free and unencumbered software released into the public domain.

impl core::str::FromStr for Decimal {
    type Err = rust_decimal::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Self(rust_decimal::Decimal::from_str(input)?))
    }
}
