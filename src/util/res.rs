use std::collections::VecDeque;
use std::ops::DerefMut;
use std::sync::{Arc, LockResult, Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard};

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
}

impl<T> Clone for Res<T> {
    fn clone(&self) -> Self {
        Res { inner: self.inner.clone() }
    }
}

pub struct Global<T: Send + Sync> {
    inner: RwLock<T>,
    change_queue: Mutex<VecDeque<Box<dyn GlobalChange<T> + Send + Sync>>>
}

impl<T: Send + Sync + 'static> Global<T> {
    pub fn new(inner: T) -> Global<T> {
        Global {
            inner: RwLock::new(inner),
            change_queue: Mutex::new(VecDeque::new()),
        }
    }

    pub fn queue_change(&self, change: fn(&mut T)) {
        let mut queue = self.change_queue.lock().unwrap();
        queue.push_back(Box::new(change));
    }

    pub fn execute_changes(&self) {
        let mut queue = self.change_queue.lock().unwrap();
        let mut inner = self.inner.write().unwrap();
        while let Some(change) = queue.pop_front() {
            change.execute(inner.deref_mut());
        }
    }

    pub fn read(&self) -> LockResult<RwLockReadGuard<'_, T>> {
        self.inner.read()
    }

    pub fn write(&self) -> LockResult<RwLockWriteGuard<'_, T>> {
        self.inner.write()
    }
}

trait GlobalChange<T: Send + Sync> {
    fn execute(&self, t: &mut T);
}

impl<T: Send + Sync> GlobalChange<T> for fn(&mut T) {
    fn execute(&self, t: &mut T) {
        self(t)
    }
}