extern crate nohash_hasher;

// Re-export hash functions.
pub use nohash_hasher::{
    BuildNoHashHasher,
    NoHashHasher,
    IsEnabled,
};

use std::collections::{
    HashMap,
    HashSet,
};


/// A `HashMap` using a default `NoHashHasher`.
///
/// # Example
///
/// ```
/// # use nohash::{
/// #     NoHashHasher,
/// #     NoHashHashMap,
/// #     IsEnabled,
/// # };
/// #
/// #[derive(PartialEq, Eq)]
/// struct NodeIndex(usize);
///
/// impl NodeIndex {
///     fn new(index: usize) -> Self { Self { 0: index } }  
/// }
/// 
/// impl std::hash::Hash for NodeIndex {
///     fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
///         hasher.write_usize(self.0)
///     }
/// }
///
/// impl IsEnabled for NodeIndex {}
///
/// let mut m: NoHashHashMap<NodeIndex, String> = NoHashHashMap::default();
///
/// m.insert(NodeIndex(0), String::from("root"));
/// m.insert(NodeIndex(1), String::from("child"));
///
/// assert_eq!(m.get(&NodeIndex(1)), Some(&String::from("child")));
/// assert_eq!(m.get(&NodeIndex(0)), Some(&String::from("root")));
/// ```
#[cfg(feature = "std")]
pub type NoHashHashMap<K, V> = HashMap<K, V, BuildNoHashHasher<K>>;

/// A `HashSet` using a default `NoHashHasher`.
///
/// # Example
///
/// ```
/// # use nohash::{
/// #     NoHashHasher,
/// #     NoHashHashSet,
/// #     IsEnabled,
/// # };
/// #
/// #[derive(PartialEq, Eq)]
/// struct NodeIndex(usize);
///
/// impl NodeIndex {
///     fn new(index: usize) -> Self { Self { 0: index } }  
/// }
/// 
/// impl IsEnabled for NodeIndex {}
///
/// let mut m: NoHashHashSet<usize> = NoHashHashSet::default();
///
/// m.insert(1);
/// m.insert(0);
///
/// assert!(m.contains(&1));
/// assert!(m.contains(&0));
/// ```
#[cfg(feature = "std")]
pub type NoHashHashSet<V> = HashSet<V, BuildNoHashHasher<V>>;

#[cfg(test)]
mod tests {
    use super::*;
    
    
}
