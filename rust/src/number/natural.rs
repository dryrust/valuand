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
