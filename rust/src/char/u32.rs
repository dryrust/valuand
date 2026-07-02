// This is free and unencumbered software released into the public domain.

impl TryFrom<u32> for Char {
    type Error = core::char::CharTryFromError;

    fn try_from(input: u32) -> Result<Self, Self::Error> {
        Ok(Self(input.try_into()?))
    }
}

impl From<Char> for u32 {
    fn from(input: Char) -> Self {
        input.0 as _
    }
}

impl From<&Char> for u32 {
    fn from(input: &Char) -> Self {
        input.0 as _
    }
}
