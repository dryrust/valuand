// This is free and unencumbered software released into the public domain.

impl From<i64> for Rational {
    fn from(input: i64) -> Self {
        Self::from(Integer::from(input))
    }
}

impl From<(i64, i64)> for Rational {
    fn from(input: (i64, i64)) -> Self {
        Self::from((Integer::from(input.0), Integer::from(input.1)))
    }
}
