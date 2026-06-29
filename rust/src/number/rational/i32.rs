// This is free and unencumbered software released into the public domain.

impl From<i32> for Rational {
    fn from(input: i32) -> Self {
        Self::from(Integer::from(input))
    }
}

impl From<(i32, i32)> for Rational {
    fn from(input: (i32, i32)) -> Self {
        Self::from((Integer::from(input.0), Integer::from(input.1)))
    }
}
