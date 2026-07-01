// This is free and unencumbered software released into the public domain.

#[cfg(feature = "bson")]
impl From<F64> for bson::Bson {
    fn from(input: F64) -> Self {
        input.into_bson()
    }
}
