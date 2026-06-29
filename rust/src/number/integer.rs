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

include!("integer/i8.rs");
include!("integer/i16.rs");
include!("integer/i32.rs");
include!("integer/i64.rs");
include!("integer/i128.rs");
include!("integer/i256.rs");
include!("integer/big.rs");

#[cfg(feature = "decimal")]
impl From<Integer> for rust_decimal::Decimal {
    fn from(input: Integer) -> Self {
        input.into()
    }
}
