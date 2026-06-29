// This is free and unencumbered software released into the public domain.

impl From<i128> for Rational {
    fn from(input: i128) -> Self {
        Self::from(Integer::from(input))
    }
}

impl From<(i128, i128)> for Rational {
    fn from(input: (i128, i128)) -> Self {
        Self::from((Integer::from(input.0), Integer::from(input.1)))
    }
}
