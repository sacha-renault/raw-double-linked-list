//! Queue implementation providing first-in, first-out (FIFO) element access.
//!
//! This module implements a Queue data structure that follows FIFO semantics.
//! Elements are added to one end of the queue and removed from the opposite end,
//! ensuring the first element added will be the first one removed.
//!
//! The implementation uses a linked list as its underlying storage mechanism,
//! providing efficient operations for both insertion and removal.

use super::single_ended_collection::SingleEndedCollection ;
use crate::list::linked_list::List;
use std::fmt::Debug;

#[derive(Default)]
pub struct Queue<T> {
    inner: List<T>,
}

/// A first-in, first-out (FIFO) collection.
///
/// `Queue` stores elements in the order they were added and removes them
/// in the same order. This implementation uses a linked list as its
/// underlying storage, providing O(1) complexity for both push and pop operations.
///
/// # Type Parameters
///
/// * `T` - The type of elements stored in the queue.
///
/// # Examples
///
/// ```
/// use raw_double_linked_list::prelude::{SingleEndedCollection, Queue};
///
/// let mut queue = Queue::new();
///
/// // Add elements to the queue
/// queue.push("first");
/// queue.push("second");
///
/// // Elements come out in the same order they went in
/// assert_eq!(queue.pop(), Some("first"));
/// assert_eq!(queue.pop(), Some("second"));
/// ```
impl<T> Queue<T> {
    /// Creates a new, empty queue.
    ///
    /// # Returns
    ///
    /// A new `Queue<T>` with no elements.
    pub fn new() -> Self {
        Self { inner: List::new() }
    }
}

impl<T> SingleEndedCollection  for Queue<T> {
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

impl<T: Debug> Debug for Queue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.inner.iter().rev()).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_queue_test() {
        // Create a new queue
        let mut queue = Queue::<i32>::new();

        // Test that a new queue is empty
        assert_eq!(queue.len(), 0);

        // Test push operation
        queue.push(1);
        assert_eq!(queue.len(), 1);

        queue.push(2);
        assert_eq!(queue.len(), 2);

        queue.push(3);
        assert_eq!(queue.len(), 3);

        // Test pop operation - should return items in FIFO order
        assert_eq!(queue.pop(), Some(1));
        assert_eq!(queue.len(), 2);

        assert_eq!(queue.pop(), Some(2));
        assert_eq!(queue.len(), 1);

        assert_eq!(queue.pop(), Some(3));
        assert_eq!(queue.len(), 0);

        // Test popping from an empty queue
        assert_eq!(queue.pop(), None);
        assert_eq!(queue.len(), 0);
    }

    #[test]
    fn test_default() {
        // Test the Default implementation
        let queue: Queue<i32> = Queue::default();
        assert_eq!(queue.len(), 0);

        // Check that default is equivalent to new
        let new_queue = Queue::<i32>::new();
        assert_eq!(queue.len(), new_queue.len());
    }

    #[test]
    fn test_debug_implementation() {
        let mut queue = Queue::<i32>::new();

        // Test Debug formatting on empty queue
        let debug_empty = format!("{:?}", queue);
        assert_eq!(debug_empty, "[]");

        // Test push and debug display
        queue.push(1);
        queue.push(2);
        queue.push(3);

        // Assert
        let debug_filled = format!("{:?}", queue);
        assert_eq!(debug_filled, "[1, 2, 3]");
    }

    #[test]
    fn test_queue_ordering() {
        // Verify FIFO (First-In-First-Out) behavior
        let mut queue = Queue::<String>::new();

        queue.push("first".to_string());
        queue.push("second".to_string());
        queue.push("third".to_string());

        // Items should come out in the opposite order they were pushed (FIFO)
        assert_eq!(queue.pop(), Some("first".to_string()));
        assert_eq!(queue.pop(), Some("second".to_string()));
        assert_eq!(queue.pop(), Some("third".to_string()));
    }
}
