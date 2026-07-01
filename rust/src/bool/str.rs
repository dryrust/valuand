// This is free and unencumbered software released into the public domain.

impl core::str::FromStr for Bool {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "0" | "false" => Ok(Self(false)),
            "1" | "true" => Ok(Self(true)),
            _ => Err(()),
        }
    }
}
