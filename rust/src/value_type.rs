// This is free and unencumbered software released into the public domain.

use super::Value;
use core::{
    any::{Any, TypeId},
    fmt::Debug,
};

/// A type discriminator for a [Value].
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ValueType {
    #[default]
    Unit,
    Bool,
    I32,
    U32,
    I64,
    U64,
    Other(TypeId),
}

impl ValueType {
    pub fn type_id(&self) -> TypeId {
        use ValueType::*;
        match self {
            Unit => TypeId::of::<()>(),
            Bool => TypeId::of::<bool>(),
            I32 => TypeId::of::<i32>(),
            U32 => TypeId::of::<u32>(),
            I64 => TypeId::of::<i64>(),
            U64 => TypeId::of::<u64>(),
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
            Value::I32(_) => I32,
            Value::U32(_) => U32,
            Value::I64(_) => I64,
            Value::U64(_) => U64,
            Value::Other(x) => Other(x.type_id()),
        }
    }
}
