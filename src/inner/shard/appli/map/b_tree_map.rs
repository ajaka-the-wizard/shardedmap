use std::{collections::BTreeMap, hash::Hash};

use crate::inner::shard::traits::shardable::ShardableMap;

impl<K, V> ShardableMap<K, V> for BTreeMap<K, V>
where
    K: Eq + Hash + Ord,
{
    fn new() -> Self {
        BTreeMap::new()
    }
    fn get(&self, key: &K) -> Option<&V> {
        self.get(key)
    }
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.insert(key, value)
    }
    fn remove(&mut self, key: &K) -> Option<V> {
        self.remove(key)
    }
    fn len(&self) -> usize {
        self.len()
    }
    fn clear(&mut self) {
        self.clear()
    }
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}
