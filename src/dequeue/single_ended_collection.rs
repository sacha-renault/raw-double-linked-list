//! Data structures module implementing queue and stack interfaces.
//!
//! This module provides trait definitions and implementations for common data structures
//! that support queue and stack operations. The primary trait is `Dequeue`, which defines
//! the minimum functionality required for basic queue and dequeue operations.

pub trait SingleEndedCollection  {
    /// The type of elements that will be stored in this data structure.
    type Item;

    /// Removes and returns the next element from the data structure.
    ///
    /// Depending on the specific implementation, this could return:
    /// - The last element added (for stack behavior)
    /// - The first element added (for queue behavior)
    ///
    /// # Returns
    ///
    /// * `Some(Item)` - If the data structure contains at least one element
    /// * `None` - If the data structure is empty
    fn pop(&mut self) -> Option<Self::Item>;

    /// Adds a new element to the data structure.
    ///
    /// The specific behavior (where the element is added) depends on the implementation.
    ///
    /// # Parameters
    ///
    /// * `value` - The element to add to the data structure
    fn push(&mut self, value: Self::Item);

    /// Returns the current number of elements in the data structure.
    ///
    /// # Returns
    ///
    /// The number of elements currently stored in the data structure
    fn len(&self) -> usize;

    /// Checks if the data structure is empty.
    ///
    /// # Returns
    ///
    /// `true` if the data structure contains no elements, `false` otherwise
    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
