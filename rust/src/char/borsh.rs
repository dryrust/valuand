// This is free and unencumbered software released into the public domain.

#[cfg(feature = "borsh")]
impl borsh::BorshSerialize for Char {
    fn serialize<W: borsh::io::Write>(&self, writer: &mut W) -> borsh::io::Result<()> {
        let value = self.as_u32();
        value.serialize(writer)
    }
}

#[cfg(feature = "borsh")]
impl borsh::BorshDeserialize for Char {
    fn deserialize_reader<R: borsh::io::Read>(reader: &mut R) -> borsh::io::Result<Self> {
        let value = u32::deserialize_reader(reader)?;
        Ok(value
            .try_into()
            .map_err(|_| borsh::io::Error::other("CharTryFromError"))?)
    }
}
