// This is free and unencumbered software released into the public domain.

impl From<i8> for Integer {
    fn from(input: i8) -> Self {
        Self::I8(input)
    }
}

impl TryFrom<Integer> for i8 {
    type Error = ();

    fn try_from(input: Integer) -> Result<Self, Self::Error> {
        Self::try_from(&input)
    }
}

impl TryFrom<&Integer> for i8 {
    type Error = ();

    fn try_from(input: &Integer) -> Result<Self, Self::Error> {
        input.to_i8().ok_or(())
    }
}
