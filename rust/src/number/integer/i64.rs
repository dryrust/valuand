// This is free and unencumbered software released into the public domain.

impl From<i64> for Integer {
    fn from(input: i64) -> Self {
        Self::I64(input)
    }
}

impl TryFrom<Integer> for i64 {
    type Error = ();

    fn try_from(input: Integer) -> Result<Self, Self::Error> {
        Self::try_from(&input)
    }
}

impl TryFrom<&Integer> for i64 {
    type Error = ();

    fn try_from(input: &Integer) -> Result<Self, Self::Error> {
        input.to_i64().ok_or(())
    }
}
