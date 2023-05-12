use info::{ScalarType, TypeInfo};


/// This module provides type info structures and enums
pub mod info;


/// The main trait of this library which allows introspection on both structs and enums
pub trait Introspectable {
    /// Returns type info for the implemented type.
    fn introspect() -> info::TypeInfo;
}

// Implementation of Introspectable for tuple sizes up to 16
// A macro is used for this

#[macro_export]
macro_rules! impl_introspectable {
    ($($i:ident),+) => {
        impl<$($i: Introspectable),+> Introspectable for ($($i,)+) {
            fn introspect() -> info::TypeInfo {
                info::TypeInfo::Compound(
                    info::CompoundType::Tuple{
                        fields: vec![$($i::introspect(),)+]
                    }
                )
            }
        }
    };
}

impl_introspectable!(T1);
impl_introspectable!(T1, T2);
impl_introspectable!(T1, T2, T3);
impl_introspectable!(T1, T2, T3, T4);
impl_introspectable!(T1, T2, T3, T4, T5);
impl_introspectable!(T1, T2, T3, T4, T5, T6);
impl_introspectable!(T1, T2, T3, T4, T5, T6, T7);
impl_introspectable!(T1, T2, T3, T4, T5, T6, T7, T8);
impl_introspectable!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_introspectable!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
impl_introspectable!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
impl_introspectable!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
impl_introspectable!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
impl_introspectable!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
impl_introspectable!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
impl_introspectable!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);

// Implementation of Introspectable for pointer types

impl<T: Introspectable> Introspectable for &T {
    fn introspect() -> info::TypeInfo {
        info::TypeInfo::Pointer(
            info::PointerType::Reference(Box::new(T::introspect()))
        )
    }
}

impl<T: Introspectable> Introspectable for &mut T {
    fn introspect() -> info::TypeInfo {
        info::TypeInfo::Pointer(
            info::PointerType::MutReference(Box::new(T::introspect()))
        )
    }
}

impl<T: Introspectable> Introspectable for *const T {
    fn introspect() -> info::TypeInfo {
        info::TypeInfo::Pointer(
            info::PointerType::ConstPointer(Box::new(T::introspect()))
        )
    }
}

impl<T: Introspectable> Introspectable for *mut T {
    fn introspect() -> info::TypeInfo {
        info::TypeInfo::Pointer(
            info::PointerType::MutPointer(Box::new(T::introspect()))
        )
    }
}



// Implementation of Introspectable for all scalar types

impl Introspectable for bool {
    fn introspect() -> TypeInfo {
        TypeInfo::Scalar(ScalarType::Bool)
    }
}

impl Introspectable for i8 {
    fn introspect() -> TypeInfo {
        TypeInfo::Scalar(ScalarType::I8)
    }
}

impl Introspectable for i16 {
    fn introspect() -> TypeInfo {
        TypeInfo::Scalar(ScalarType::I16)
    }
}

impl Introspectable for i32 {
    fn introspect() -> TypeInfo {
        TypeInfo::Scalar(ScalarType::I32)
    }
}

impl Introspectable for i64 {
    fn introspect() -> TypeInfo {
        TypeInfo::Scalar(ScalarType::I64)
    }
}

impl Introspectable for i128 {
    fn introspect() -> TypeInfo {
        TypeInfo::Scalar(ScalarType::I128)
    }
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

impl Introspectable for u64 {
    fn introspect() -> TypeInfo {
        TypeInfo::Scalar(ScalarType::U64)
    }
}

impl Introspectable for u128 {
    fn introspect() -> TypeInfo {
        TypeInfo::Scalar(ScalarType::U128)
    }
}

impl Introspectable for f32 {
    fn introspect() -> TypeInfo {
        TypeInfo::Scalar(ScalarType::F32)
    }
}

impl Introspectable for f64 {
    fn introspect() -> TypeInfo {
        TypeInfo::Scalar(ScalarType::F64)
    }
}

impl Introspectable for char {
    fn introspect() -> TypeInfo {
        TypeInfo::Scalar(ScalarType::Char)
    }
}