// This is free and unencumbered software released into the public domain.

impl From<char> for Char {
    fn from(input: char) -> Self {
        Self(input)
    }
}

impl From<Char> for char {
    fn from(input: Char) -> Self {
        input.0
    }
}

impl From<&Char> for char {
    fn from(input: &Char) -> Self {
        input.0
    }
}
