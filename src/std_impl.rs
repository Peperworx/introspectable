/// Implements Introspectable for several core standard library types.

use std::collections::{
    VecDeque, LinkedList, HashSet, BTreeSet, HashMap, BTreeMap,
};

use crate::{Introspectable, info::{TypeInfo, SpecializedType}};


impl<T: Introspectable> Introspectable for Vec<T> {
    fn introspect() -> TypeInfo {
        TypeInfo::Specialized(
            SpecializedType::Vec {
                type_info: Box::new(T::introspect())
            }
        )
    }
}

impl<T: Introspectable> Introspectable for VecDeque<T> {
    fn introspect() -> TypeInfo {
        TypeInfo::Specialized(
            SpecializedType::VecDeque {
                type_info: Box::new(T::introspect())
            }
        )
    }
}

impl<T: Introspectable> Introspectable for LinkedList<T> {
    fn introspect() -> TypeInfo {
        TypeInfo::Specialized(
            SpecializedType::LinkedList {
                type_info: Box::new(T::introspect())
            }
        )
    }
}

impl<K: Introspectable, V: Introspectable> Introspectable for HashMap<K, V> {
    fn introspect() -> TypeInfo {
        TypeInfo::Specialized(
            SpecializedType::HashMap {
                key_type: Box::new(K::introspect()),
                value_type: Box::new(V::introspect()),
            }
        )
    }
}

impl<K: Introspectable, V: Introspectable> Introspectable for BTreeMap<K, V> {
    fn introspect() -> TypeInfo {
        TypeInfo::Specialized(
            SpecializedType::BTreeMap {
                key_type: Box::new(K::introspect()),
                value_type: Box::new(V::introspect()),
            }
        )
    }
}

impl<T: Introspectable> Introspectable for HashSet<T> {
    fn introspect() -> TypeInfo {
        TypeInfo::Specialized(
            SpecializedType::HashSet {
                type_info: Box::new(T::introspect())
            }
        )
    }
}

impl<T: Introspectable> Introspectable for BTreeSet<T> {
    fn introspect() -> TypeInfo {
        TypeInfo::Specialized(
            SpecializedType::BTreeSet {
                type_info: Box::new(T::introspect())
            }
        )
    }
}