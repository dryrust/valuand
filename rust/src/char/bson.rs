// This is free and unencumbered software released into the public domain.

#[cfg(feature = "bson")]
impl From<Char> for bson::Bson {
    fn from(input: Char) -> Self {
        input.into_bson()
    }
}
