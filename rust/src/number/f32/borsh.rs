// This is free and unencumbered software released into the public domain.

#[cfg(feature = "borsh")]
impl borsh::BorshSerialize for F32 {
    fn serialize<W: borsh::io::Write>(&self, writer: &mut W) -> borsh::io::Result<()> {
        let value = self.as_f32();
        value.serialize(writer)
    }
}

#[cfg(feature = "borsh")]
impl borsh::BorshDeserialize for F32 {
    fn deserialize_reader<R: borsh::io::Read>(reader: &mut R) -> borsh::io::Result<Self> {
        let value = f32::deserialize_reader(reader)?;
        Ok(Self(value.into()))
    }
}
