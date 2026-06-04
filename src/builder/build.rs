use std::{
    collections::{BTreeMap, HashMap},
    hash::Hash,
    sync::RwLock,
};

use crate::inner::{map::sharded_map::ShardedMap, shard::traits::shardable::ShardableMap};

pub struct Builder;

impl Builder {
    pub fn new_default_lock_with_hashmap<K, V>(
        num_shards: usize,
    ) -> ShardedMap<K, V, HashMap<K, V>, RwLock<HashMap<K, V>>>
    where
        K: Hash + Eq,
        V: Clone,
    {
        ShardedMap::<K, V, HashMap<K, V>, RwLock<HashMap<K, V>>>::new(num_shards)
    }

    pub fn new_default_lock_with_btreemap<K, V>(
        num_shards: usize,
    ) -> ShardedMap<K, V, BTreeMap<K, V>, RwLock<BTreeMap<K, V>>>
    where
        K: Hash + Eq + Ord,
        V: Clone,
    {
        ShardedMap::<K, V, BTreeMap<K, V>, RwLock<BTreeMap<K, V>>>::new(num_shards)
    }

    pub fn new_default_lock_with_custom_map<K, V, M>(
        num_shards: usize,
    ) -> ShardedMap<K, V, M, RwLock<M>>
    where
        K: Hash + Eq,
        V: Clone,
        M: ShardableMap<K, V>,
    {
        ShardedMap::<K, V, M, RwLock<M>>::new(num_shards)
    }
}
