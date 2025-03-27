use std::fmt::Debug;
use super::dequeue::Dequeue;
use crate::list::linked_list::List;

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