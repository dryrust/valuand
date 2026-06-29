// This is free and unencumbered software released into the public domain.

/// A shorthand type alias for [`Natural`].
pub type Nat = Natural;

/// A natural number.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

include!("natural/u8.rs");
include!("natural/u16.rs");
include!("natural/u32.rs");
include!("natural/u64.rs");
include!("natural/u128.rs");
include!("natural/u256.rs");
include!("natural/big.rs");

#[cfg(feature = "decimal")]
impl From<Natural> for rust_decimal::Decimal {
    fn from(input: Natural) -> Self {
        input.into()
    }
}
