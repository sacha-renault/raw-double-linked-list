use std::fmt::Debug;
use super::dequeue::Dequeue;
use crate::list::linked_list::List;

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

impl<T: Debug> Debug for Stack<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.inner.iter()).finish()
    }
}