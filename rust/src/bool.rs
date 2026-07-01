// This is free and unencumbered software released into the public domain.

/// A boolean.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
pub struct Bool(pub bool);

impl Bool {
    pub const TRUE: Bool = Bool(true);
    pub const FALSE: Bool = Bool(false);

    pub const fn is_true(&self) -> bool {
        self.0 == true
    }

    pub const fn is_false(&self) -> bool {
        self.0 == false
    }

    pub const fn as_bool(&self) -> bool {
        self.0
    }

    pub const fn as_f32(&self) -> f32 {
        self.as_u8() as _
    }

    pub const fn as_f64(&self) -> f64 {
        self.as_u8() as _
    }

    pub const fn as_i8(&self) -> i8 {
        self.as_u8() as _
    }

    pub const fn as_u8(&self) -> u8 {
        match self.0 {
            true => 1,
            false => 0,
        }
    }

    pub const fn as_isize(&self) -> isize {
        self.as_u8() as _
    }

    pub const fn as_usize(&self) -> usize {
        self.as_u8() as _
    }

    pub const fn to_bool(&self) -> Option<bool> {
        Some(self.0)
    }

    pub const fn into_inner(self) -> bool {
        self.0
    }

    #[cfg(feature = "serde")]
    pub fn to_json(&self) -> Option<serde_json::Value> {
        Some(self.clone().into_json())
    }

    #[cfg(feature = "serde")]
    pub fn into_json(self) -> serde_json::Value {
        serde_json::Value::Bool(self.as_bool())
    }

    #[cfg(feature = "bson")]
    pub fn to_bson(&self) -> Option<bson::Bson> {
        Some(self.clone().into_bson())
    }

    #[cfg(feature = "bson")]
    pub fn into_bson(self) -> bson::Bson {
        bson::Bson::Boolean(self.as_bool())
    }
}

impl core::fmt::Display for Bool {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            false => write!(f, "false"),
            true => write!(f, "true"),
        }
    }
}

impl<T> From<&T> for Bool
where
    T: Clone + Into<Self>,
{
    fn from(t: &T) -> Self {
        t.clone().into()
    }
}

impl From<bool> for Bool {
    fn from(input: bool) -> Self {
        Self(input)
    }
}

impl From<Bool> for bool {
    fn from(input: Bool) -> Self {
        input.0
    }
}

impl From<&Bool> for bool {
    fn from(input: &Bool) -> Self {
        input.0
    }
}

include!("bool/str.rs");

include!("bool/bson.rs");
