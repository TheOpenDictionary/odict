use std::collections::HashMap;

/// Convert a Vec of one type to a Vec of another type that implements From<T>
pub fn convert_vec<T, U: From<T>>(items: Vec<T>) -> Vec<U> {
    items.into_iter().map(Into::into).collect()
}

/// Convert an optional value to another type that implements From<T>
pub fn convert_option<T, U: From<T>>(item: Option<T>) -> Option<U> {
    item.map(Into::into)
}

/// Convert a HashMap where values need conversion
pub fn convert_hash_values<K: Clone + Eq + std::hash::Hash, V, U: From<V>>(
    map: HashMap<K, V>,
) -> HashMap<K, U> {
    map.into_iter().map(|(k, v)| (k, v.into())).collect()
}
