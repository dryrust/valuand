// This is free and unencumbered software released into the public domain.

impl From<i16> for Integer {
    fn from(input: i16) -> Self {
        Self::I16(input)
    }
}

impl TryFrom<Integer> for i16 {
    type Error = ();

    fn try_from(input: Integer) -> Result<Self, Self::Error> {
        Self::try_from(&input)
    }
}

impl TryFrom<&Integer> for i16 {
    type Error = ();

    fn try_from(input: &Integer) -> Result<Self, Self::Error> {
        input.to_i16().ok_or(())
    }
}
