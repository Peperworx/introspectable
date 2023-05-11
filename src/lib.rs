
/// This module provides type info structures and enums
pub mod info;


/// The main trait of this library which allows introspection on both structs and enums
pub trait Introspect {
    /// Returns type info for the implemented type.
    fn introspect() -> info::TypeInfo;
}
