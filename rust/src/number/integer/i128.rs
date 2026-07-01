// This is free and unencumbered software released into the public domain.

impl From<i128> for Integer {
    fn from(input: i128) -> Self {
        Self::I128(input)
    }
}

impl TryFrom<Integer> for i128 {
    type Error = ();

    fn try_from(input: Integer) -> Result<Self, Self::Error> {
        Self::try_from(&input)
    }
}

impl TryFrom<&Integer> for i128 {
    type Error = ();

    fn try_from(input: &Integer) -> Result<Self, Self::Error> {
        input.to_i128().ok_or(())
    }
}
