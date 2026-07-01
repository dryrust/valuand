// This is free and unencumbered software released into the public domain.

#[cfg(feature = "bson")]
impl From<F32> for bson::Bson {
    fn from(input: F32) -> Self {
        input.into_bson()
    }
}
