use std::collections::HashMap;

/// This enum identifies type information for a given type
/// # Warning
/// This is not implemented for pointer types and slices, which will just resolve to their pointed to data.
#[derive(Debug, Clone)]
pub enum TypeInfo {
    /// Represents a scalar type
    Scalar(ScalarType),
    /// Represents a compound type
    Compound(CompoundType),
    /// Represents a pointer type
    Pointer(PointerType),
    /// Represents a special type that is handled differently than others,
    /// including standard library types such as Vec, Box, HasmMap, etc.
    /// Only included when the specialized_std trait is enabled
    #[cfg(feature="specialized_std")]
    Specialized(SpecializedType)
}

/// This enum identifies each of the 14 primitive scalar types
#[derive(Debug, Clone, Copy)]
pub enum ScalarType {
    Bool,
    I8, I16, I32, I64, I128,
    U8, U16, U32, U64, U128,
    F32, F64,
    Char
}

impl ScalarType {
    /// Returns true if the type is an integer
    pub fn is_integer(self) -> bool {
        matches!(self, ScalarType::I8 | ScalarType::I16 | ScalarType::I32 | ScalarType::I64 | ScalarType::I128 | ScalarType::U8 | ScalarType::U16 | ScalarType::U32 | ScalarType::U64 | ScalarType::U128)
    }

    /// Returns true if the type is a floating point number
    pub fn is_float(self) -> bool {
        matches!(self, ScalarType::F32 | ScalarType::F64)
    }

    /// Returns true if the type is signed
    pub fn is_signed(self) -> bool {
        matches!(self, ScalarType::I8 | ScalarType::I16 | ScalarType::I32 | ScalarType::I64 | ScalarType::I128 | ScalarType::F32 | ScalarType::F64)
    }
}

/// This enum identifies different compound types
#[derive(Debug, Clone)]
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
    /// This variant represents an array
    Array {
        /// The type of the array
        type_info: Box<TypeInfo>,
        /// The length of the array
        length: usize,
    }
}

/// Represents an Enum variant
#[derive(Debug, Clone)]
pub enum EnumVariant {
    /// Represents a unit-like enum
    UnitVariant,
    /// Represents a tuple-like enum
    UnnamedVariant {
        /// The fields of the enum
        fields: Vec<TypeInfo>
    },
    /// Represents a named variant
    NamedVariant {
        /// The fields of the enum. Key is the name of the field, value is the type info.
        fields: HashMap<&'static str, TypeInfo>
    }
}

/// This enum identifies different pointer types
#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum SpecializedType {

}