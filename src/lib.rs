extern crate nohash_hasher;

// Re-export hash functions.
pub use nohash_hasher::*;

use std::collections::{
    HashMap,
    HashSet,
};


/// A `HashMap` using a default `NoHashHasher`.
#[cfg(feature = "std")]
pub type NoHashHashMap<K, V> = HashMap<K, V, BuildNoHashHasher<K>>;

/// A `HashSet` using a default `NoHashHasher`.
#[cfg(feature = "std")]
pub type NoHashHashSet<K, V> = HashSet<V, BuildNoHashHasher<K>>;


#[cfg(test)]
mod tests {
    use super::*;
    
    
}
