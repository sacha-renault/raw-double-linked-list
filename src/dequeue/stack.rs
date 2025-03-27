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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_stack_test() {
        // Create a new stack
        let mut stack = Stack::<i32>::new();

        // Test that a new stack is empty
        assert_eq!(stack.len(), 0);

        // Test push operation
        stack.push(1);
        assert_eq!(stack.len(), 1);

        stack.push(2);
        assert_eq!(stack.len(), 2);

        stack.push(3);
        assert_eq!(stack.len(), 3);

        // Test pop operation - should return items in LIFO order
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.len(), 2);

        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.len(), 1);

        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.len(), 0);

        // Test popping from an empty stack
        assert_eq!(stack.pop(), None);
        assert_eq!(stack.len(), 0);
    }

    #[test]
    fn test_default() {
        // Test the Default implementation
        let stack: Stack<i32> = Stack::default();
        assert_eq!(stack.len(), 0);

        // Check that default is equivalent to new
        let new_stack = Stack::<i32>::new();
        assert_eq!(stack.len(), new_stack.len());
    }

    #[test]
    fn test_debug_implementation() {
        let mut stack = Stack::<i32>::new();

        // Test Debug formatting on empty stack
        let debug_empty = format!("{:?}", stack);
        assert_eq!(debug_empty, "[]");

        // Test Debug formatting with elements
        stack.push(1);
        stack.push(2);
        stack.push(3);

        let debug_filled = format!("{:?}", stack);
        assert_eq!(debug_filled, "[1, 2, 3]");
    }
}