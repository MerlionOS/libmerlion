//! Collections (std::collections equivalent).
//!
//! Re-exports from alloc.

pub use alloc::collections::BTreeMap;
pub use alloc::collections::BTreeSet;
pub use alloc::collections::BinaryHeap;
pub use alloc::collections::LinkedList;
pub use alloc::collections::VecDeque;

// HashMap/HashSet require a hasher — provide a simple one.
// In real std, these use RandomState. We use FNV for simplicity.

/// Simple FNV-1a hasher for HashMap/HashSet.
pub struct FnvHasher(u64);

impl FnvHasher {
    pub fn new() -> Self { Self(0xcbf29ce484222325) }
}

impl core::hash::Hasher for FnvHasher {
    fn finish(&self) -> u64 { self.0 }
    fn write(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.0 ^= b as u64;
            self.0 = self.0.wrapping_mul(0x100000001b3);
        }
    }
}

/// Build hasher for HashMap.
pub struct FnvBuildHasher;

impl core::hash::BuildHasher for FnvBuildHasher {
    type Hasher = FnvHasher;
    fn build_hasher(&self) -> FnvHasher { FnvHasher::new() }
}

/// HashMap using FNV hasher (no_std compatible).
pub type HashMap<K, V> = hashbrown_inline::HashMap<K, V, FnvBuildHasher>;
/// HashSet using FNV hasher.
pub type HashSet<K> = hashbrown_inline::HashSet<K, FnvBuildHasher>;

/// Inline minimal hashmap (no external dependency).
pub mod hashbrown_inline {
    use alloc::vec::Vec;
    use core::hash::{Hash, BuildHasher, Hasher};

    pub struct HashMap<K, V, S> {
        entries: Vec<Option<(K, V)>>,
        hasher: S,
    }

    impl<K: Hash + Eq, V, S: BuildHasher + Default> HashMap<K, V, S> {
        pub fn new() -> Self {
            Self { entries: Vec::new(), hasher: S::default() }
        }

        pub fn insert(&mut self, key: K, value: V) -> Option<V> {
            for entry in &mut self.entries {
                if let Some((ref k, _)) = entry {
                    if k == &key {
                        let old = entry.take().unwrap().1;
                        *entry = Some((key, value));
                        return Some(old);
                    }
                }
            }
            self.entries.push(Some((key, value)));
            None
        }

        pub fn get(&self, key: &K) -> Option<&V> {
            for entry in &self.entries {
                if let Some((ref k, ref v)) = entry {
                    if k == key { return Some(v); }
                }
            }
            None
        }

        pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
            for entry in &mut self.entries {
                if let Some((ref k, ref mut v)) = entry {
                    if k == key { return Some(v); }
                }
            }
            None
        }

        pub fn remove(&mut self, key: &K) -> Option<V> {
            for entry in &mut self.entries {
                if let Some((ref k, _)) = entry {
                    if k == key {
                        return entry.take().map(|(_, v)| v);
                    }
                }
            }
            None
        }

        pub fn contains_key(&self, key: &K) -> bool { self.get(key).is_some() }
        pub fn len(&self) -> usize { self.entries.iter().filter(|e| e.is_some()).count() }
        pub fn is_empty(&self) -> bool { self.len() == 0 }
    }

    impl<K, V, S: Default> Default for HashMap<K, V, S> {
        fn default() -> Self { Self { entries: Vec::new(), hasher: S::default() } }
    }

    pub struct HashSet<K, S> {
        map: HashMap<K, (), S>,
    }

    impl<K: Hash + Eq, S: BuildHasher + Default> HashSet<K, S> {
        pub fn new() -> Self { Self { map: HashMap::new() } }
        pub fn insert(&mut self, key: K) -> bool { self.map.insert(key, ()).is_none() }
        pub fn contains(&self, key: &K) -> bool { self.map.contains_key(key) }
        pub fn remove(&mut self, key: &K) -> bool { self.map.remove(key).is_some() }
        pub fn len(&self) -> usize { self.map.len() }
    }

    impl<K, S: Default> Default for HashSet<K, S> {
        fn default() -> Self { Self { map: HashMap::default() } }
    }
}

impl Default for FnvBuildHasher {
    fn default() -> Self { Self }
}
