// This is free and unencumbered software released into the public domain.

impl From<isize> for Rational {
    fn from(input: isize) -> Self {
        Self::from(Integer::from(input))
    }
}

impl From<(isize, isize)> for Rational {
    fn from(input: (isize, isize)) -> Self {
        Self::from((Integer::from(input.0), Integer::from(input.1)))
    }
}
