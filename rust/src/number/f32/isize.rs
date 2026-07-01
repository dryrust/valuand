// This is free and unencumbered software released into the public domain.

impl From<i8> for F32 {
    fn from(input: i8) -> Self {
        Self((input as f32).into())
    }
}

impl From<i16> for F32 {
    fn from(input: i16) -> Self {
        Self((input as f32).into())
    }
}

impl From<i32> for F32 {
    fn from(input: i32) -> Self {
        Self((input as f32).into())
    }
}

impl From<i64> for F32 {
    fn from(input: i64) -> Self {
        Self((input as f32).into())
    }
}

impl From<i128> for F32 {
    fn from(input: i128) -> Self {
        Self((input as f32).into())
    }
}

impl From<isize> for F32 {
    fn from(input: isize) -> Self {
        Self((input as f32).into())
    }
}
