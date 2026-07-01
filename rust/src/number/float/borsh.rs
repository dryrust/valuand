// This is free and unencumbered software released into the public domain.

#[cfg(feature = "borsh")]
impl borsh::BorshSerialize for Float {
    fn serialize<W: borsh::io::Write>(&self, _writer: &mut W) -> borsh::io::Result<()> {
        todo!() // TODO
    }
}

#[cfg(feature = "borsh")]
impl borsh::BorshDeserialize for Float {
    fn deserialize_reader<R: borsh::io::Read>(_reader: &mut R) -> borsh::io::Result<Self> {
        todo!() // TODO
    }
}
