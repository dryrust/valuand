// This is free and unencumbered software released into the public domain.

impl From<i16> for Rational {
    fn from(input: i16) -> Self {
        Self::from(Integer::from(input))
    }
}

impl From<(i16, i16)> for Rational {
    fn from(input: (i16, i16)) -> Self {
        Self::from((Integer::from(input.0), Integer::from(input.1)))
    }
}
