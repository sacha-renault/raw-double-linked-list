//! Last-in, first-out (LIFO) collection implementation.
//!
//! Provides a stack data structure where elements are added and removed
//! from the same end, ensuring the most recently added element is the
//! first to be removed.

use super::single_ended_collection::SingleEndedCollection ;
use crate::list::linked_list::List;
use std::fmt::Debug;

/// A last-in, first-out (LIFO) collection.
///
/// `Stack<T>` implements the standard stack behavior where elements are pushed
/// and popped from the same end. The implementation utilizes a linked list as
/// its underlying storage mechanism, offering O(1) complexity for push and pop
/// operations.
///
/// # Type Parameters
///
/// * `T` - The type of elements stored in the stack
///
/// # Examples
///
/// ```
/// # use raw_double_linked_list::prelude::{SingleEndedCollection, Stack};
/// let mut stack = Stack::new();
///
/// // Add elements to the stack
/// stack.push(10);
/// stack.push(20);
///
/// // Elements come out in reverse order (LIFO)
/// assert_eq!(stack.pop(), Some(20));
/// assert_eq!(stack.pop(), Some(10));
/// assert_eq!(stack.pop(), None); // Empty stack returns None
/// ```
#[derive(Default)]
pub struct Stack<T> {
    inner: List<T>,
}

impl<T> Stack<T> {
    /// Creates a new, empty stack.
    ///
    /// # Returns
    ///
    /// A new `Stack<T>` instance with zero elements.
    pub fn new() -> Self {
        Self { inner: List::new() }
    }
}

impl<T> SingleEndedCollection  for Stack<T> {
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
        assert!(!stack.is_empty());

        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.len(), 0);
        assert!(stack.is_empty());

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
