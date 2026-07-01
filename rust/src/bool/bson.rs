// This is free and unencumbered software released into the public domain.

#[cfg(feature = "bson")]
impl From<Bool> for bson::Bson {
    fn from(input: Bool) -> Self {
        input.into_bson()
    }
}
