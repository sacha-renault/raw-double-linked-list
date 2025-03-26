use super::dequeue::Dequeue;
use crate::List;

#[derive(Default)]
pub struct Queue<T> {
    inner: List<T>
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self { inner: List::new() }
    }
}

impl<T> Dequeue for Queue<T> {
    type Item = T;

    fn pop(&mut self) -> Option<Self::Item> {
        self.inner.pop_back()
    }

    fn push(&mut self, value: Self::Item) {
        self.inner.push_front(value);
    }

    fn len(&self) -> usize {
        self.inner.len()
    }
}