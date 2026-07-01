// This is free and unencumbered software released into the public domain.

impl From<i32> for Integer {
    fn from(input: i32) -> Self {
        Self::I32(input)
    }
}

impl TryFrom<Integer> for i32 {
    type Error = ();

    fn try_from(input: Integer) -> Result<Self, Self::Error> {
        Self::try_from(&input)
    }
}

impl TryFrom<&Integer> for i32 {
    type Error = ();

    fn try_from(input: &Integer) -> Result<Self, Self::Error> {
        input.to_i32().ok_or(())
    }
}
