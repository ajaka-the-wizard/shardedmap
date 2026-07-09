use std::hash::{DefaultHasher, Hash, Hasher};

use crate::inner::shard::traits::shardable::ShardableMap;

struct Node<K, V> {
    key: K,
    value: V,
    next: Option<Box<Node<K, V>>>,
}

pub struct MyCustomMap<K, V>
where
    K: Hash + Ord + Eq,
{
    buckets: Vec<Option<Box<Node<K, V>>>>,
    size: usize,
    length: usize,
}

impl<K, V> MyCustomMap<K, V>
where
    K: Hash + Ord + Eq,
{
    pub fn new() -> Self {
        let mut buckets = Vec::with_capacity(100);
        for _ in 0..100 {
            buckets.push(None);
        }
        Self {
            buckets,
            size: 100,
            length: 0,
        }
    }

    fn hash(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() % (self.size as u64)) as usize
    }

    fn insert_recursively(node: &mut Node<K, V>, new_node: Box<Node<K, V>>) -> Option<V> {
        if node.key == new_node.key {
            return Some(std::mem::replace(&mut node.value, new_node.value));
        }

        match node.next.as_mut() {
            Some(n) => Self::insert_recursively(n, new_node),
            None => {
                node.next = Some(new_node);
                None
            }
        }
    }
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let index = self.hash(&key);
        let new_node = Box::new(Node {
            key,
            value,
            next: None,
        });
        let outcome = match &mut self.buckets[index] {
            Some(n) => Self::insert_recursively(n, new_node),
            None => {
                self.buckets[index] = Some(new_node);
                None
            }
        };
        self.length += 1;
        outcome
    }

    fn get_recursively<'a>(node: &'a Option<Box<Node<K, V>>>, key: &K) -> Option<&'a V> {
        match node {
            None => None,
            Some(n) if n.key == *key => Some(&n.value),
            Some(n) => Self::get_recursively(&n.next, key),
        }
    }
    pub fn get<'a>(&'a self, key: &K) -> Option<&'a V> {
        let index = self.hash(&key);
        let node: &Option<Box<Node<K, V>>> = &self.buckets[index];
        Self::get_recursively(node, key)
    }

    pub fn contains(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    fn remove_recursively(node: &mut Box<Node<K, V>>, key: &K) -> Option<V> {
        match node.next.as_mut() {
            None => None,
            Some(n) if n.key == *key => {
                let mut removed_node = node.next.take().unwrap();
                node.next = removed_node.next.take();
                Some(removed_node.value)
            }
            Some(n) => Self::remove_recursively(n, key),
        }
    }
    pub fn remove(&mut self, key: &K) -> Option<V> {
        let index = self.hash(&key);

        let outcome = if let Some(n) = self.buckets[index].as_mut() {
            if n.key == *key {
                let mut removed_node = self.buckets[index].take().unwrap();
                self.buckets[index] = removed_node.next.take();
                return Some(removed_node.value);
            } else {
                Self::remove_recursively(n, key)
            }
        } else {
            return None;
        };
        if outcome.is_some() {
            self.length -= 1;
        }
        outcome
    }
    pub fn len(&self) -> usize {
        self.length
    }
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
    pub fn clear(&mut self) {
        let mut buckets = Vec::with_capacity(100);
        for _ in 0..100 {
            buckets.push(None);
        }
        self.buckets = buckets;
    }
}

impl<K, V> ShardableMap<K, V> for MyCustomMap<K, V>
where
    K: Hash + Eq + Ord,
{
    fn new() -> Self {
        MyCustomMap::new()
    }
    fn get(&self, key: &K) -> Option<&V> {
        self.get(key)
    }
    fn insert(&mut self, key: K, value: V) {
        self.insert(key, value);
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

impl<K, V> Default for MyCustomMap<K, V>
where
    K: Hash + Ord + Eq,
{
    fn default() -> Self {
        MyCustomMap::new()
    }
}
