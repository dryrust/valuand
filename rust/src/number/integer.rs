// This is free and unencumbered software released into the public domain.

/// An integer number.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Integer {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
}

impl Integer {
    pub fn as_i128(&self) -> i128 {
        match self {
            Self::I8(i) => *i as _,
            Self::I16(i) => *i as _,
            Self::I32(i) => *i as _,
            Self::I64(i) => *i as _,
            Self::I128(i) => *i,
        }
    }
}

impl From<i8> for Integer {
    fn from(input: i8) -> Self {
        Self::I8(input)
    }
}

impl From<i16> for Integer {
    fn from(input: i16) -> Self {
        Self::I16(input)
    }
}

impl From<i32> for Integer {
    fn from(input: i32) -> Self {
        Self::I32(input)
    }
}

impl From<i64> for Integer {
    fn from(input: i64) -> Self {
        Self::I64(input)
    }
}

impl From<i128> for Integer {
    fn from(input: i128) -> Self {
        Self::I128(input)
    }
}

impl From<Integer> for i128 {
    fn from(input: Integer) -> Self {
        input.as_i128()
    }
}
