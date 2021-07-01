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
/// let mut m: NoHashHashSet<usize> = NoHashHashSet::default();
///
/// m.insert(1);
/// m.insert(0);
/// m.insert(2);
///
/// assert!(m.contains(&2));
/// assert!(m.contains(&1));
/// assert!(m.contains(&0));
/// ```
#[cfg(feature = "std")]
pub type NoHashHashSet<V> = HashSet<V, BuildNoHashHasher<V>>;

#[cfg(test)]
mod tests {
    use super::*;
    
    
    #[test]
    fn test_hash_map() {
        let mut m = NoHashHashMap::default();
        let expected = vec![
            (9, 10), 
            (8, 20), 
            (7, 30), 
            (6, 40), 
            (5, 50), 
            (4, 60), 
            (3, 70), 
            (2, 80), 
            (1, 90), 
            (0, 100)
        ];
        for (k, v) in expected.iter().copied() {
            m.insert(k, v);
        }

        assert!(expected.iter().all(|(k, v)| { m.get(k) == Some(v) }));
    }

    #[test]
    fn test_hash_map_integer() {
        #[derive(Copy, Clone, PartialEq, Eq)]
        struct KeyType(usize);

        impl KeyType {
            fn new(index: usize) -> Self { Self { 0: index } }
        }

        impl std::hash::Hash for KeyType {
            fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
                hasher.write_usize(self.0)
            }
        }

        impl IsEnabled for KeyType {}

        let mut m = NoHashHashMap::default();
        let expected = vec![
            (KeyType::new(9), 10), 
            (KeyType::new(8), 20), 
            (KeyType::new(7), 30), 
            (KeyType::new(6), 40), 
            (KeyType::new(5), 50), 
            (KeyType::new(4), 60), 
            (KeyType::new(3), 70), 
            (KeyType::new(2), 80), 
            (KeyType::new(1), 90), 
            (KeyType::new(0), 100)
        ];
        for (k, v) in expected.iter().copied() {
            m.insert(k, v);
        }

        assert!(expected.iter().all(|(k, v)| { m.get(k) == Some(v) }));
    }

    #[test]
    fn test_hash_set() {
        let mut m = NoHashHashSet::default();
        let expected = vec![
            (9, 10), 
            (8, 20), 
            (7, 30), 
            (6, 40), 
            (5, 50), 
            (4, 60), 
            (3, 70), 
            (2, 80), 
            (1, 90), 
            (0, 100)
        ];
        for (_, v) in expected.iter().copied() {
            m.insert(v);
        }

        assert!(expected.iter().all(|(_, v)| { m.contains(v) }));
    }
}

