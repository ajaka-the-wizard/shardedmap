use crate::inner::shard::traits::shardable::{ShardLock, ShardableMap};
use std::{
    hash::{DefaultHasher, Hash, Hasher},
    marker::PhantomData,
};

pub struct ShardedMap<K, V, M, L>
where
    M: ShardableMap<K, V>,
    L: ShardLock<M>,
{
    shards: Vec<L>,
    shard_count: usize,
    _marker: PhantomData<(K, V, M)>,
}

impl<K, V, M, L> ShardedMap<K, V, M, L>
where
    K: Hash + Eq,
    M: ShardableMap<K, V>,
    L: ShardLock<M>,
{
    pub fn new(num_shards: usize) -> Self {
        let mut shards = Vec::with_capacity(num_shards);
        for _ in 0..num_shards {
            shards.push(L::new(M::new()));
        }
        Self {
            shards,
            shard_count: num_shards,
            _marker: PhantomData,
        }
    }

    fn hash(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() % (self.shard_count as u64)) as usize
    }

    pub fn insert(&self, key: K, value: V) {
        let shard_index = self.hash(&key);
        let shard = &self.shards[shard_index];
        let mut shard_write_access = shard.write();
        shard_write_access.insert(key, value);
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let shard_index = self.hash(&key);
        let shard = &self.shards[shard_index];
        let shard_read_access: <L as ShardLock<M>>::Guard<'_> = shard.read();
        let value = shard_read_access.get(key);
        return value;
    }
    pub fn confirm(&self) {
        println!("Yep")
    }
}
