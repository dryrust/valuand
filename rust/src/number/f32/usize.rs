// This is free and unencumbered software released into the public domain.

impl From<u8> for F32 {
    fn from(input: u8) -> Self {
        Self((input as f32).into())
    }
}

impl From<u16> for F32 {
    fn from(input: u16) -> Self {
        Self((input as f32).into())
    }
}

impl From<u32> for F32 {
    fn from(input: u32) -> Self {
        Self((input as f32).into())
    }
}

impl From<u64> for F32 {
    fn from(input: u64) -> Self {
        Self((input as f32).into())
    }
}

impl From<u128> for F32 {
    fn from(input: u128) -> Self {
        Self((input as f32).into())
    }
}

impl From<usize> for F32 {
    fn from(input: usize) -> Self {
        Self((input as f32).into())
    }
}
