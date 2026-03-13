use std::sync::Arc;

use crossbeam_queue::ArrayQueue;

#[derive(Clone)]
pub struct SpscRingBuffer<T, const N: usize> {
    inner: Arc<ArrayQueue<T>>,
}

impl<T, const N: usize> SpscRingBuffer<T, N> {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(ArrayQueue::new(N)),
        }
    }

    pub fn push(&self, item: T) -> Result<(), T> {
        self.inner.push(item)
    }

    pub fn pop(&self) -> Option<T> {
        self.inner.pop()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }
}
