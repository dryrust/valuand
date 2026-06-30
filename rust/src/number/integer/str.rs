// This is free and unencumbered software released into the public domain.

impl core::str::FromStr for Integer {
    type Err = core::num::ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Self::I128(i128::from_str(input)?))
    }
}
