// This is free and unencumbered software released into the public domain.

/// A natural number.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Natural {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
}

impl Natural {
    pub fn as_u128(&self) -> u128 {
        match self {
            Self::U8(i) => *i as _,
            Self::U16(i) => *i as _,
            Self::U32(i) => *i as _,
            Self::U64(i) => *i as _,
            Self::U128(i) => *i,
        }
    }
}

impl From<u8> for Natural {
    fn from(input: u8) -> Self {
        Self::U8(input)
    }
}

impl From<u16> for Natural {
    fn from(input: u16) -> Self {
        Self::U16(input)
    }
}

impl From<u32> for Natural {
    fn from(input: u32) -> Self {
        Self::U32(input)
    }
}

impl From<u64> for Natural {
    fn from(input: u64) -> Self {
        Self::U64(input)
    }
}

impl From<u128> for Natural {
    fn from(input: u128) -> Self {
        Self::U128(input)
    }
}
