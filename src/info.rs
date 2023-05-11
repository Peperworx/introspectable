use std::collections::HashMap;

/// This enum identifies type information for a given type
/// # Warning
/// This is not implemented for pointer types and slices, which will just resolve to their pointed to data.
pub enum TypeInfo {
    /// Represents a scalar type
    Scalar(ScalarType),
    /// Represents a compound type
    Compound(CompoundType),
    /// Represents a special type that is handled differently than others,
    /// including standard library types such as Vec, Box, HasmMap, etc.
    /// Only included when the specialized_std trait is enabled
    #[cfg(feature="specialized_std")]
    Specialized(SpecializedType)
}

/// This enum identifies each of the 14 primitive scalar types
pub enum ScalarType {
    Bool(bool),
    I8(i8), I16(i16), I32(i32), I64(i64), I128(i128),
    U8(u8), U16(u16), U32(u32), U64(u64), U128(u128),
    F32(f32), F64(f64),
    Char(char)
}

/// This enum identifies different compound types
pub enum CompoundType {
    /// This variant represents a structure and its fields
    Struct {
        /// The name of the structure
        name: &'static str,
        /// The structure's fields. Key is the name of the field, value is the typeinfo
        fields: HashMap<&'static str, TypeInfo>
    },
    /// This variant represents a tuple
    Tuple {
        /// The tuple's field's type info
        fields: Vec<TypeInfo>,
    },
    /// This variant represents an enum
    Enum {
        /// The enum's name
        name: &'static str,
        /// The enum's variants. Key is the name of the variant, value is the EnumVariant type.
        variants: HashMap<&'static str, EnumVariant>
    },

}

/// Represents an Enum variant
pub enum EnumVariant {
    /// Represents a unit-like enum
    UnitEnum,
    /// Represents a tuple-like enum
    TupleEnum {
        /// The fields of the enum
        fields: Vec<TypeInfo>
    },
    /// Represents a struct-like enum
    StructEnum {
        /// The fields of the enum. Key is the name of the field, value is the type info.
        fields: HashMap<&'static str, TypeInfo>
    }
}

/// This enum identifies different pointer types
pub enum PointerType {
    /// Represents a reference
    Reference(Box<TypeInfo>),
    /// Represents a mutable reference
    MutReference(Box<TypeInfo>),
    /// Represents a const pointer
    ConstPointer(Box<TypeInfo>),
    /// References a mutable pointer
    MutPointer(Box<TypeInfo>)
}

pub enum SpecializedType {

}