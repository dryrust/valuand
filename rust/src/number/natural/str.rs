// This is free and unencumbered software released into the public domain.

impl core::str::FromStr for Natural {
    type Err = core::num::ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Self::U128(u128::from_str(input)?))
    }
}
