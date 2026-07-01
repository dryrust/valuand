// This is free and unencumbered software released into the public domain.

#[cfg(feature = "bson")]
impl From<Decimal> for bson::Bson {
    fn from(input: Decimal) -> Self {
        input.into_bson().unwrap()
    }
}
