// This is free and unencumbered software released into the public domain.

use super::ValueType;
use core::{any::TypeId, fmt::Debug};

#[doc(hidden)]
#[derive(Debug)]
pub struct T;

/// A sum type that can hold values of any static type.
#[derive(Debug, Default)]
pub enum Value<T: Debug = self::T> {
    #[default]
    Unit,
    Bool(bool),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    Other(T),
}

impl<T: Debug + 'static> Value<T> {
    pub fn r#type(&self) -> ValueType {
        self.into()
    }

    pub fn type_id(&self) -> TypeId {
        self.r#type().type_id()
    }
}

impl<T: Debug> From<()> for Value<T> {
    fn from(_input: ()) -> Self {
        Self::Unit
    }
}

impl<T: Debug> From<bool> for Value<T> {
    fn from(input: bool) -> Self {
        Self::Bool(input)
    }
}

impl<T: Debug> From<i32> for Value<T> {
    fn from(input: i32) -> Self {
        Self::I32(input)
    }
}

impl<T: Debug> From<u32> for Value<T> {
    fn from(input: u32) -> Self {
        Self::U32(input)
    }
}

impl<T: Debug> From<i64> for Value<T> {
    fn from(input: i64) -> Self {
        Self::I64(input)
    }
}

impl<T: Debug> From<u64> for Value<T> {
    fn from(input: u64) -> Self {
        Self::U64(input)
    }
}

impl<T: Debug> From<Value<T>> for () {
    fn from(input: Value<T>) -> Self {
        From::<&Value<T>>::from(&input)
    }
}

impl<T: Debug> From<&Value<T>> for () {
    fn from(input: &Value<T>) -> Self {
        match input {
            Value::Unit => (),
            _ => unreachable!(),
        }
    }
}

impl<T: Debug> From<Value<T>> for bool {
    fn from(input: Value<T>) -> Self {
        From::<&Value<T>>::from(&input)
    }
}

impl<T: Debug> From<&Value<T>> for bool {
    fn from(input: &Value<T>) -> Self {
        match input {
            Value::Bool(value) => *value,
            _ => unreachable!(),
        }
    }
}

impl<T: Debug> From<Value<T>> for i32 {
    fn from(input: Value<T>) -> Self {
        From::<&Value<T>>::from(&input)
    }
}

impl<T: Debug> From<&Value<T>> for i32 {
    fn from(input: &Value<T>) -> Self {
        match input {
            Value::I32(value) => *value,
            _ => unreachable!(),
        }
    }
}

impl<T: Debug> From<Value<T>> for u32 {
    fn from(input: Value<T>) -> Self {
        From::<&Value<T>>::from(&input)
    }
}

impl<T: Debug> From<&Value<T>> for u32 {
    fn from(input: &Value<T>) -> Self {
        match input {
            Value::U32(value) => *value,
            _ => unreachable!(),
        }
    }
}

impl<T: Debug> From<Value<T>> for i64 {
    fn from(input: Value<T>) -> Self {
        From::<&Value<T>>::from(&input)
    }
}

impl<T: Debug> From<&Value<T>> for i64 {
    fn from(input: &Value<T>) -> Self {
        match input {
            Value::I32(value) => *value as _,
            Value::U32(value) => *value as _,
            Value::I64(value) => *value,
            _ => unreachable!(),
        }
    }
}

impl<T: Debug> From<Value<T>> for u64 {
    fn from(input: Value<T>) -> Self {
        From::<&Value<T>>::from(&input)
    }
}

impl<T: Debug> From<&Value<T>> for u64 {
    fn from(input: &Value<T>) -> Self {
        match input {
            Value::I32(value) => *value as _,
            Value::U32(value) => *value as _,
            Value::U64(value) => *value,
            _ => unreachable!(),
        }
    }
}
