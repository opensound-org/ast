use std::{
    borrow::Borrow,
    collections::{BTreeMap, HashMap},
    hash::{BuildHasher, Hash},
};
use thiserror::Error;

/// Error returned by `MapExt::replace_key`.
#[derive(Error, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ReplaceKeyErr {
    #[error("replace_key: the old key does not exist")]
    /// The old key does not exist.
    OldKeyNotExist,
    #[error("replace_key: the new key is already occupied")]
    /// The new key is already occupied.
    NewKeyOccupied,
}

/// Some general extensions to `Maps` (such as
/// [`HashMap`](https://doc.rust-lang.org/stable/std/collections/struct.HashMap.html),
/// [`BTreeMap`](https://doc.rust-lang.org/stable/std/collections/struct.BTreeMap.html),
/// [`IndexMap`](https://docs.rs/indexmap/latest/indexmap/map/struct.IndexMap.html)).
pub trait MapExt<K, Q: ?Sized = K> {
    /// Replace an existing key with a new (non-existing) one.
    ///
    /// If k1 does not exist, return `Err(ReplaceKeyErr::OldKeyNotExist)`.
    ///
    /// If k1 exists and k2 also exists, return `Err(ReplaceKeyErr::NewKeyOccupied)`.
    ///
    /// Otherwise, return `Ok(())` after the replacement is completed.
    fn replace_key(&mut self, k1: &Q, k2: K) -> Result<(), ReplaceKeyErr>
    where
        K: Borrow<Q>;
}

impl<K, Q, V, S> MapExt<K, Q> for HashMap<K, V, S>
where
    K: Eq + Hash + Borrow<Q>,
    Q: Hash + Eq + ?Sized,
    S: BuildHasher,
{
    fn replace_key(&mut self, k1: &Q, k2: K) -> Result<(), ReplaceKeyErr> {
        if !self.contains_key(k1) {
            return Err(ReplaceKeyErr::OldKeyNotExist);
        }

        if self.contains_key(k2.borrow()) {
            return Err(ReplaceKeyErr::NewKeyOccupied);
        }

        let v = self.remove(k1).expect("this should be unreachable");
        self.insert(k2, v);
        Ok(())
    }
}

impl<K, Q, V> MapExt<K, Q> for BTreeMap<K, V>
where
    K: Borrow<Q> + Ord,
    Q: Ord + ?Sized,
{
    fn replace_key(&mut self, k1: &Q, k2: K) -> Result<(), ReplaceKeyErr> {
        if !self.contains_key(k1) {
            return Err(ReplaceKeyErr::OldKeyNotExist);
        }

        if self.contains_key(k2.borrow()) {
            return Err(ReplaceKeyErr::NewKeyOccupied);
        }

        let v = self.remove(k1).expect("this should be unreachable");
        self.insert(k2, v);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replace_key_hashmap() {
        let mut map = HashMap::new();

        map.insert("k1".to_string(), 123);
        map.insert("k2".to_string(), 456);

        assert_eq!(map["k1"], 123);
        assert_eq!(map["k2"], 456);
        assert_eq!(
            map.replace_key("k3", "k2".to_string()),
            Err(ReplaceKeyErr::OldKeyNotExist)
        );
        assert_eq!(
            map.replace_key("k3", "k4".to_string()),
            Err(ReplaceKeyErr::OldKeyNotExist)
        );
        assert_eq!(
            map.replace_key("k1", "k2".to_string()),
            Err(ReplaceKeyErr::NewKeyOccupied)
        );
        assert_eq!(map.replace_key("k1", "k3".to_string()), Ok(()));
        assert!(!map.contains_key("k1"));
        assert_eq!(map["k3"], 123);
        assert_eq!(map["k2"], 456);
    }

    #[test]
    fn replace_key_btreemap() {
        let mut map = BTreeMap::new();

        map.insert("k1".to_string(), 123);
        map.insert("k2".to_string(), 456);

        assert_eq!(map["k1"], 123);
        assert_eq!(map["k2"], 456);
        assert_eq!(
            map.replace_key("k3", "k2".to_string()),
            Err(ReplaceKeyErr::OldKeyNotExist)
        );
        assert_eq!(
            map.replace_key("k3", "k4".to_string()),
            Err(ReplaceKeyErr::OldKeyNotExist)
        );
        assert_eq!(
            map.replace_key("k1", "k2".to_string()),
            Err(ReplaceKeyErr::NewKeyOccupied)
        );
        assert_eq!(map.replace_key("k1", "k3".to_string()), Ok(()));
        assert!(!map.contains_key("k1"));
        assert_eq!(map["k3"], 123);
        assert_eq!(map["k2"], 456);
    }
}
