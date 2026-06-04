use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::inner::shard::traits::shardable::ShardLock;

impl<T> ShardLock<T> for RwLock<T> {
    type Guard<'a>
        = RwLockReadGuard<'a, T>
    where
        Self: 'a;
    type WriteGuard<'a>
        = RwLockWriteGuard<'a, T>
    where
        Self: 'a;

    fn new(val: T) -> Self {
        RwLock::new(val)
    }
    #[inline]
    fn read(&self) -> Self::Guard<'_> {
        self.read().expect("Read lock Poisoned")
    }
    #[inline]
    fn write(&self) -> Self::WriteGuard<'_> {
        self.write().expect("Write lock Poisoned")
    }
}
