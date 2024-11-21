use std::{
    borrow::Borrow,
    collections::{BTreeMap, HashMap},
    hash::{BuildHasher, Hash},
};

/// Some general extensions to `Maps` (such as `HashMap`, `BTreeMap`, `IndexMap`).
pub trait MapExt<K, Q: ?Sized = K> {
    /// Replace an existing key with a new one.
    ///
    /// If the key exists and is successfully replaced, return true;
    /// otherwise, return false.
    fn replace_key(&mut self, k1: &Q, k2: K) -> bool
    where
        K: Borrow<Q>;
}

impl<K, Q, V, S> MapExt<K, Q> for HashMap<K, V, S>
where
    K: Eq + Hash + Borrow<Q>,
    Q: Hash + Eq + ?Sized,
    S: BuildHasher,
{
    fn replace_key(&mut self, k1: &Q, k2: K) -> bool {
        if let Some(v) = self.remove(k1) {
            self.insert(k2, v);
            true
        } else {
            false
        }
    }
}

impl<K, Q, V> MapExt<K, Q> for BTreeMap<K, V>
where
    K: Borrow<Q> + Ord,
    Q: Ord + ?Sized,
{
    fn replace_key(&mut self, k1: &Q, k2: K) -> bool {
        if let Some(v) = self.remove(k1) {
            self.insert(k2, v);
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replace_key_hashmap() {
        let mut map = HashMap::new();
        map.insert("k1".to_string(), 123);

        assert_eq!(map["k1"], 123);
        assert!(!map.replace_key("k2", "k2".to_string()));
        assert!(map.replace_key("k1", "k2".to_string()));
        assert!(!map.contains_key("k1"));
        assert_eq!(map["k2"], 123);
    }

    #[test]
    fn replace_key_btreemap() {
        let mut map = BTreeMap::new();
        map.insert("k1".to_string(), 123);

        assert_eq!(map["k1"], 123);
        assert!(!map.replace_key("k2", "k2".to_string()));
        assert!(map.replace_key("k1", "k2".to_string()));
        assert!(!map.contains_key("k1"));
        assert_eq!(map["k2"], 123);
    }
}
