use super::dequeue::Dequeue;
use crate::List;

#[derive(Default)]
pub struct Stack<T> {
    inner: List<T>
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { inner: List::new() }
    }
}

impl<T> Dequeue for Stack<T> {
    type Item = T;

    fn pop(&mut self) -> Option<Self::Item> {
        self.inner.pop_back()
    }

    fn push(&mut self, value: Self::Item) {
        self.inner.push_back(value);
    }

    fn len(&self) -> usize {
        self.inner.len()
    }
}