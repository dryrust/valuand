// This is free and unencumbered software released into the public domain.

use super::Value;
use core::{
    any::{Any, TypeId},
    fmt::Debug,
};

/// A type discriminator for a [`Value`].
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ValueType {
    #[default]
    Unit,
    Bool,
    #[cfg(feature = "number")]
    Number,
    Other(TypeId),
}

impl ValueType {
    pub fn type_id(&self) -> TypeId {
        use ValueType::*;
        match self {
            Unit => TypeId::of::<()>(),
            Bool => TypeId::of::<bool>(),
            #[cfg(feature = "number")]
            Number => TypeId::of::<super::Real>(),
            Other(type_id) => *type_id,
        }
    }
}

impl<T: Debug + 'static> From<Value<T>> for ValueType {
    fn from(input: Value<T>) -> Self {
        From::<&Value<T>>::from(&input)
    }
}

impl<T: Debug + 'static> From<&Value<T>> for ValueType {
    fn from(input: &Value<T>) -> Self {
        use ValueType::*;
        match input {
            Value::Unit => Unit,
            Value::Bool(_) => Bool,
            #[cfg(feature = "number")]
            Value::Number(_) => Number,
            Value::Other(x) => Other(x.type_id()),
        }
    }
}
