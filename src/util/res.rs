use std::sync::{Arc, LockResult, RwLock, RwLockReadGuard, RwLockWriteGuard};

/// Holds a resource. Glorified Arc<Mutex<T>>.
#[derive(Debug, Default)]
pub struct Res<T> {
    inner: Arc<RwLock<T>>
}

impl<T> Res<T> {
    pub fn new(inner: T) -> Self {
        Res { inner: Arc::new(RwLock::new(inner)) }
    }

    pub fn read(&self) -> LockResult<RwLockReadGuard<'_, T>> {
        self.inner.read()
    }

    pub fn write(&self) -> LockResult<RwLockWriteGuard<'_, T>> {
        self.inner.write()
    }

    pub fn clone(&self) -> Self {
        Res { inner: self.inner.clone() }
    }
}

