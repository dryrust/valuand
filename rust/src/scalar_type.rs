// This is free and unencumbered software released into the public domain.

use super::Scalar;
use core::{
    any::{Any, TypeId},
    fmt::Debug,
};

/// A type discriminator for a [`Scalar`].
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ScalarType {
    #[default]
    Unit,

    Bool,

    #[cfg(feature = "number")]
    Number,

    Char,

    Other(TypeId),
}

impl ScalarType {
    pub fn type_id(&self) -> TypeId {
        use ScalarType::*;
        match self {
            Unit => TypeId::of::<()>(),
            Bool => TypeId::of::<bool>(),
            #[cfg(feature = "number")]
            Number => TypeId::of::<super::Real>(),
            Char => TypeId::of::<char>(),
            Other(type_id) => *type_id,
        }
    }
}

impl<T: Debug + 'static> From<Scalar<T>> for ScalarType {
    fn from(input: Scalar<T>) -> Self {
        From::<&Scalar<T>>::from(&input)
    }
}

impl<T: Debug + 'static> From<&Scalar<T>> for ScalarType {
    fn from(input: &Scalar<T>) -> Self {
        use ScalarType::*;
        match input {
            Scalar::Unit => Unit,
            Scalar::Bool(_) => Bool,
            #[cfg(feature = "number")]
            Scalar::Number(_) => Number,
            Scalar::Char(_) => Char,
            Scalar::Other(x) => Other(x.type_id()),
        }
    }
}
