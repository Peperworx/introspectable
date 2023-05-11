use info::{ScalarType, TypeInfo};


/// This module provides type info structures and enums
pub mod info;


/// The main trait of this library which allows introspection on both structs and enums
pub trait Introspectable {
    /// Returns type info for the implemented type.
    fn introspect() -> info::TypeInfo;
}


impl Introspectable for u8 {
    fn introspect() -> TypeInfo {
        TypeInfo::Scalar(ScalarType::U8)
    }
}

impl Introspectable for u16 {
    fn introspect() -> TypeInfo {
        TypeInfo::Scalar(ScalarType::U16)
    }
}

impl Introspectable for u32 {
    fn introspect() -> TypeInfo {
        TypeInfo::Scalar(ScalarType::U32)
    }
}
