use std::ops::{Deref, DerefMut};

pub trait ShardableMap<K, V>: Default {
    fn new() -> Self;
    fn get(&self, key: &K) -> Option<&V>;
    fn insert(&mut self, key: K, value: V) -> Option<V>;
    fn remove(&mut self, key: &K) -> Option<V>;
    fn len(&self) -> usize;
    fn clear(&mut self);
    fn is_empty(&self) -> bool;
}

pub trait ShardLock<T> {
    type Guard<'a>: Deref<Target = T> + 'a
    where
        Self: 'a;
    type WriteGuard<'a>: DerefMut<Target = T> + 'a
    where
        Self: 'a;
    fn new(val: T) -> Self
    where
        Self: Sized;
    fn read(&self) -> Self::Guard<'_>;
    fn write(&self) -> Self::WriteGuard<'_>;
}
