//! A doubly-linked list implementation with safe memory management.
//
//! This module provides a general-purpose doubly-linked list with O(1) operations
//! for adding or removing elements at either end. The list maintains pointers to
//! both ends to enable efficient bidirectional access.
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::{Index, IndexMut};

use super::errors::Errors;
use super::list_item::{DoubleLinkedListItem, ItemPtr};
use super::list_iter::ListIter;
use super::list_sort::{merge_sort_by, UpdateListBound};
use super::list_utility::{find_index_through, get_ptr_starting_point, Side};

/// A doubly-linked list with pointers to both ends.
///
/// This generic list implementation allows for O(1) operations on both ends
/// and provides efficient iteration. All heap allocations are properly managed
/// to prevent memory leaks.
#[derive(Default)]
pub struct List<T> {
    start: Option<ItemPtr<T>>,
    end: Option<ItemPtr<T>>,
    len: usize,
}

impl<T> List<T> {
    /// Creates a new empty list.
    ///
    /// # Returns
    ///
    /// A new `List<T>` with no elements.
    pub fn new() -> Self {
        Self {
            start: None,
            end: None,
            len: 0,
        }
    }

    /// Adds an element to the end of the list.
    ///
    /// # Parameters
    ///
    /// * `value` - The value to add to the list
    pub fn push_back(&mut self, value: T) {
        // Instanciate a heap alloc item and obtain a raw mutable ptr to it
        let new_item = Box::new(DoubleLinkedListItem {
            value,
            next: None,
            previous: None,
        });
        let raw_ptr = Box::into_raw(new_item);
        self.len += 1;

        // start and end or both Some or both None
        // If they're none, we have to init both
        match self.start {
            None => {
                self.start = Some(raw_ptr);
                self.end = Some(raw_ptr);
            }
            Some(_) => {
                let previous_end_opt = self.end.replace(raw_ptr);
                if let Some(previous_end) = previous_end_opt {
                    unsafe {
                        (*previous_end).next = Some(raw_ptr);
                        (*raw_ptr).previous = Some(previous_end);
                    }
                }
            }
        }
    }

    /// Adds an element to the beginning of the list.
    ///
    /// # Parameters
    ///
    /// * `value` - The value to add to the list
    pub fn push_front(&mut self, value: T) {
        // Instanciate a heap alloc item and obtain a raw mutable ptr to it
        let new_item = Box::new(DoubleLinkedListItem {
            value,
            next: None,
            previous: None,
        });
        let raw_ptr = Box::into_raw(new_item);
        self.len += 1;

        // start and end or both Some or both None
        // If they're none, we have to init both
        match self.start {
            None => {
                self.start = Some(raw_ptr);
                self.end = Some(raw_ptr);
            }
            Some(_) => {
                let previous_start_opt = self.start.replace(raw_ptr);
                if let Some(previous_start) = previous_start_opt {
                    unsafe {
                        (*previous_start).previous = Some(raw_ptr);
                        (*raw_ptr).next = Some(previous_start);
                    }
                }
            }
        }
    }

    /// Inserts an element at the specified index.
    ///
    /// # Parameters
    ///
    /// * `index` - The index at which to insert the element
    /// * `value` - The value to insert
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the insertion was successful
    /// * `Err(Errors::OutOfBounds)` if the index is greater than the list length
    /// * `Err(Errors::InternalError)` if an internal error occurred
    ///
    /// # Note
    ///
    /// If `index == 0`, this is equivalent to `push_front()`.
    /// If `index == self.len()`, this is equivalent to `push_back()`.
    pub fn insert(&mut self, index: usize, value: T) -> Result<(), Errors> {
        // Check if it couldn't be replace with push back or front
        if index == 0 {
            self.push_front(value);
            return Ok(());
        } else if index == self.len {
            self.push_back(value);
            return Ok(());
        } else if index > self.len {
            return Err(Errors::OutOfBounds);
        }

        // Instanciate a heap alloc item and obtain a raw mutable ptr to it
        let new_item = Box::new(DoubleLinkedListItem {
            value,
            next: None,
            previous: None,
        });
        let raw_ptr = Box::into_raw(new_item);

        // get the before ptr
        let before_ptr_opt = self._get_ptr_at_index(index - 1);

        // We check that if we can reach the current index
        // Here we no next and previous are NOT none since
        // We already handled case front and back
        if let Some(before_ptr) = before_ptr_opt {
            unsafe {
                // Replace the next pointer of before_ptr with raw_ptr, getting the after_ptr
                let after_ptr = (*before_ptr).next.replace(raw_ptr).unwrap();
                (*after_ptr).previous = Some(raw_ptr);

                // Connect new node to nodes before and after
                (*raw_ptr).previous = Some(before_ptr);
                (*raw_ptr).next = Some(after_ptr);
            }

            // Increment the len
            self.len += 1;
            Ok(())
        } else {
            Err(Errors::InternalError)
        }
    }

    /// Removes and returns the first element of the list.
    ///
    /// # Returns
    ///
    /// * `Some(T)` containing the value if the list is not empty
    /// * `None` if the list is empty
    pub fn pop_front(&mut self) -> Option<T> {
        // Get the front node
        let front_ptr = self.start?;

        // Update the start pointer
        self.start = unsafe { (*front_ptr).next };

        // If there's a new start, update its previous pointer
        if let Some(new_start) = self.start {
            unsafe {
                (*new_start).previous = None;
            }
        } else {
            // List is now empty
            self.end = None;
            self.start = None;
        }

        // Decrement length
        self.len -= 1;

        // Convert the raw pointer back to a Box and return the value
        unsafe {
            let box_item = Box::from_raw(front_ptr);
            Some(box_item.value)
        }
    }

    /// Removes and returns the last element of the list.
    ///
    /// # Returns
    ///
    /// * `Some(T)` containing the value if the list is not empty
    /// * `None` if the list is empty
    pub fn pop_back(&mut self) -> Option<T> {
        // Get the front node
        let back_ptr = self.end?;

        // Update the start pointer
        self.end = unsafe { (*back_ptr).previous };

        // If there's a new start, update its previous pointer
        if let Some(new_end) = self.end {
            unsafe {
                (*new_end).next = None;
            }
        } else {
            // List is now empty
            self.end = None;
            self.start = None;
        }

        // Decrement length
        self.len -= 1;

        // Convert the raw pointer back to a Box and return the value
        unsafe {
            let box_item = Box::from_raw(back_ptr);
            Some(box_item.value)
        }
    }

    /// Returns the current number of elements in the list.
    /// O(1) operation since list keep track of the number of element
    ///
    /// # Returns
    ///
    /// The number of elements in the list
    pub fn len(&self) -> usize {
        self.len
    }

    /// Reverses the list in place.
    ///
    /// This operation swaps the start and end pointers and reverses
    /// all internal links between nodes.
    pub fn reverse(&mut self) {
        // Don't do anything for empty lists
        if self.len == 0 {
            return;
        }

        // Start on the begin ptr and iterate until we reach the end
        let mut current = self.start;

        // Swap start and end pointers
        std::mem::swap(&mut self.start, &mut self.end);

        // Iterate through each node
        while let Some(ptr) = current {
            // Save the next node before we change any pointers
            let next_node = unsafe { (*ptr).next };

            // Swap next and previous pointers for the current node
            unsafe {
                std::mem::swap(&mut (*ptr).next, &mut (*ptr).previous);
            }

            // Move to the next node (which was saved before the swap)
            current = next_node;
        }
    }

    /// Creates an iterator over the list elements.
    ///
    /// # Returns
    ///
    /// A bidirectional iterator for traversing the list
    pub fn iter(&self) -> ListIter<T> {
        ListIter {
            left: self.start,
            right: self.end,
            _phantom: PhantomData,
        }
    }

    /// Retrieves a pointer to the node at the specified index.
    ///
    /// This is an internal helper method used by other list methods.
    ///
    /// # Parameters
    ///
    /// * `index` - The index of the node to retrieve
    ///
    /// # Returns
    ///
    /// * `Some(ItemPtr<T>)` if the index is within bounds
    /// * `None` if the index is out of bounds or the list is empty
    fn _get_ptr_at_index(&self, index: usize) -> Option<ItemPtr<T>> {
        // Early exit condition
        if self.len == 0 || index >= self.len {
            return None;
        }

        // Start by finding where we should starting from
        let side = get_ptr_starting_point(index, self.len);

        // Get the appropriate starting pointer based on side
        let raw_ptr = match side {
            Side::Left => self.start,
            Side::Right => self.end,
        }?;

        // retrieve the associated node
        find_index_through(raw_ptr, index, self.len, &side)
    }

    /// Returns a reference to the element at the specified index.
    ///
    /// # Parameters
    ///
    /// * `index` - The index of the element to retrieve
    ///
    /// # Returns
    ///
    /// * `Some(&T)` if the index is within bounds
    /// * `None` if the index is out of bounds
    pub fn get(&self, index: usize) -> Option<&T> {
        let raw_ptr = self._get_ptr_at_index(index);
        raw_ptr.map(|ptr| unsafe { &(*ptr).value })
    }

    /// Returns a mutable reference to the element at the specified index.
    ///
    /// # Parameters
    ///
    /// * `index` - The index of the element to retrieve
    ///
    /// # Returns
    ///
    /// * `Some(&mut T)` if the index is within bounds
    /// * `None` if the index is out of bounds
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        let raw_ptr = self._get_ptr_at_index(index);
        raw_ptr.map(|ptr| unsafe { &mut (*ptr).value })
    }

    /// Checks if the list is empty.
    ///
    /// # Returns
    ///
    /// `true` if the list contains no elements, `false` otherwise
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Removes all elements from the list.
    ///
    /// This method calls `pop_front()` repeatedly until the list is empty.
    pub fn clear(&mut self) {
        while self.pop_front().is_some() {}
    }

    /// Returns a reference to the first element of the list.
    ///
    /// # Returns
    ///
    /// * `Some(&T)` if the list is not empty
    /// * `None` if the list is empty
    pub fn first(&self) -> Option<&T> {
        self.start.map(|ptr| unsafe { &(*ptr).value })
    }

    /// Returns a reference to the last element of the list.
    ///
    /// # Returns
    ///
    /// * `Some(&T)` if the list is not empty
    /// * `None` if the list is empty
    pub fn last(&self) -> Option<&T> {
        self.end.map(|ptr| unsafe { &(*ptr).value })
    }

    /// Concatenates another list to the end of this list.
    ///
    /// # Arguments
    ///
    /// * other - List to append to this list. Takes ownership of other.
    ///
    /// # Behavior
    ///
    /// If self is empty, it becomes other.
    /// If other is empty, nothing happens.
    /// Otherwise, appends other's nodes to the end of self.
    ///
    /// After concatenation, other is left empty but in a valid state.
    pub fn concatenate(&mut self, mut other: List<T>) {
        // First, ensure any of the list are empty (or not)
        if self.is_empty() {
            *self = other;
            return;
        } else if other.is_empty() {
            return;
        }

        // chain the other list to the current one
        self.end.map(|node| unsafe { (*node).next = other.start });
        other
            .start
            .map(|node| unsafe { (*node).previous = self.end });
        self.end = other.end;
        self.len += other.len;

        // Set other.end and other.start to none
        other.start = None;
        other.end = None;
    }

    pub fn sort_by<F>(&mut self, f: F)
    where
        F: Fn(&T, &T) -> std::cmp::Ordering + Copy,
    {
        // For list of size 1 or less, the list is
        // already sorted
        if self.len <= 1 {
            return;
        }

        // call utility merge sort function
        // We can unwrap because our list isn't of size
        // Null and therefore it cannot be None
        // (otherwise the List struct has deeper problems...)
        let UpdateListBound { new_start, new_end } =
            merge_sort_by(self.start.unwrap(), self.len, f);

        // update bounds of the list
        self.start = Some(new_start);
        self.end = Some(new_end);
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        // Start on the begin ptr and iterate until we arrive to the end
        let mut ptr_opt = self.start;

        // Iterate
        while let Some(ptr) = ptr_opt {
            // get next
            let next = unsafe { (*ptr).next };

            // Drop the node (taking ownership with a box will drop it)
            // At the end of the iteration
            unsafe {
                let _ = Box::from_raw(ptr);
            }

            // Move to next
            ptr_opt = next;
        }

        // Cls other ptrs
        self.start = None;
        self.end = None;
    }
}

impl<A> FromIterator<A> for List<A> {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let mut new_list = List::new();

        // Iterate through original list and add each value to the new list
        for item in iter {
            new_list.push_back(item);
        }

        new_list
    }
}

impl<T: Clone> Clone for List<T> {
    fn clone(&self) -> Self {
        let mut new_list = List::new();

        // Iterate through original list and add each value to the new list
        for item in self.iter() {
            new_list.push_back(item.clone());
        }

        new_list
    }
}

impl<T> Index<usize> for List<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).expect("Index out of bounds")
    }
}

impl<T> IndexMut<usize> for List<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).expect("Index out of bounds")
    }
}

impl<T: Debug> Debug for List<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<T> Extend<T> for List<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push_back(item);
        }
    }
}

impl<T: PartialEq> PartialEq for List<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }

        self.iter().zip(other.iter()).all(|(a, b)| a == b)
    }
}

impl<T: Eq> Eq for List<T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_test() {
        // Arrange
        let list = List::<i32>::new();

        // Arrange
        assert!(list.is_empty());
        assert_eq!(list.iter().count(), 0);
    }

    #[test]
    fn push_back_test() {
        // Arrange
        let mut list = List::new();

        // Act
        list.push_back(1);
        list.push_back(2);
        let mut iter = list.iter();

        // Arrange
        assert_eq!(list.len(), 2);
        assert_eq!(list.iter().count(), 2);
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn push_front_test() {
        // Arrange
        let mut list = List::new();

        // Act
        list.push_front(1);
        list.push_front(2);
        let mut iter = list.iter();

        // Arrange
        assert_eq!(list.len(), 2);
        assert_eq!(list.iter().count(), 2);
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn pop_front_test() {
        // Arrange
        let mut list = List::new();
        list.extend(0..5);

        // Arrange
        assert_eq!(list.len(), 5);
        assert_eq!(list.iter().count(), 5);
        assert_eq!(list.pop_front(), Some(0));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(4));
    }

    #[test]
    fn pop_back_test() {
        // Arrange
        let mut list = List::new();
        list.extend((0..5).rev());

        // Arrange
        assert_eq!(list.len(), 5);
        assert_eq!(list.pop_back(), Some(0));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(4));
    }

    #[test]
    fn insert_empty_test() {
        // Arrange
        let mut list = List::new();
        let _ = list.insert(0, 1);

        // Assert
        assert_eq!(list[0], 1);
    }

    #[test]
    fn insert_push_front_test() {
        // Arrange
        let mut list = List::new();
        list.push_front(1);
        let _ = list.insert(0, 0);

        // Assert
        assert_eq!(list.len(), 2);
        assert_eq!(list.iter().count(), 2);
        for (expected, real) in list.iter().zip(0..2) {
            assert_eq!(expected, &real);
        }
    }

    #[test]
    fn insert_push_back_test() {
        // Arrange
        let mut list = List::new();
        list.push_front(1);
        let _ = list.insert(1, 2);

        // Assert
        assert_eq!(list.len(), 2);
        assert_eq!(list.iter().count(), 2);
        for (expected, real) in list.iter().zip(1..3) {
            assert_eq!(expected, &real);
        }
    }

    #[test]
    fn insert_test() {
        // Arrange
        let mut list = List::new();
        list.push_back(1);
        list.push_back(3);
        let _ = list.insert(1, 2);

        // Assert
        assert_eq!(list.len(), 3);
        assert_eq!(list.iter().count(), 3);
        for (expected, real) in list.iter().zip(1..4) {
            assert_eq!(expected, &real);
        }
    }

    #[test]
    fn insert_out_of_bound_test() {
        // Arrange
        let mut list = List::new();
        list.push_back(1);
        list.push_back(3);
        let result = list.insert(3, 2);

        assert!(result.is_err());
    }

    #[test]
    fn reverse_test() {
        // Arrange
        let mut list = List::new();
        list.extend(0..5);

        // Assert
        assert_eq!(list.len(), 5);
        assert_eq!(list.iter().count(), 5);
        for (expected, real) in list.iter().zip(0..5) {
            assert_eq!(expected, &real);
        }

        // Act
        list.reverse();

        // Assert
        assert_eq!(list.len(), 5);
        assert_eq!(list.iter().count(), 5);
        let mut other = (0..5).collect::<Vec<_>>();
        other.reverse();
        for (expected, real) in list.iter().zip(other) {
            assert_eq!(expected, &real);
        }
    }

    #[test]
    fn clear_test() {
        // Arrange
        let mut list = List::new();
        list.extend(0..5);
        list.clear();

        // assert
        assert_eq!(list.len(), 0);
        assert_eq!(list.iter().count(), 0);
    }

    #[test]
    fn first_last_same_test() {
        // Arrange
        let mut list = List::new();
        list.push_back(1);

        // assert
        assert_eq!(list.first(), Some(&1));
        assert_eq!(list.last(), Some(&1));
        assert_eq!(list.iter().count(), 1);
    }

    #[test]
    fn first_last_different_test() {
        // Arrange
        let mut list = List::new();
        list.push_back(1);
        list.push_back(2);

        // assert
        assert_eq!(list.first(), Some(&1));
        assert_eq!(list.last(), Some(&2));
        assert_eq!(list.iter().count(), 2);
    }

    #[test]
    fn first_last_empty_test() {
        // Arrange
        let list = List::<i32>::new();

        // assert
        assert_eq!(list.first(), None);
        assert_eq!(list.last(), None);
        assert_eq!(list.iter().count(), 0);
    }

    #[test]
    fn from_iterator_test() {
        // Arrange
        let list = (0..5).collect::<List<_>>();

        // assert
        assert_eq!(list.first(), Some(&0));
        assert_eq!(list.last(), Some(&4));
        assert_eq!(list.iter().count(), 5);
        assert_eq!(list.len(), 5);
    }

    #[test]
    fn clone_add_test() {
        // Arrange
        let mut list = List::new();
        list.push_back(0);
        list.push_back(1);
        let other_list = list.clone();
        list.push_back(2);

        // assert
        assert_eq!(list.first(), other_list.first());
        assert_ne!(list.last(), other_list.last());
        assert_eq!(list.len(), other_list.len() + 1);
        assert_eq!(list.iter().count(), other_list.iter().count() + 1);
    }

    #[test]
    fn partial_eq_test() {
        // Arrange
        let list1 = (0..5).collect::<List<_>>();
        let list2 = (0..5).collect::<List<_>>();

        // Assert
        assert_eq!(list1, list2);
    }

    #[test]
    fn partial_eq_not_same_len_test() {
        // Arrange
        let list1 = (0..5).collect::<List<_>>();
        let list2 = (0..4).collect::<List<_>>();

        // Assert
        assert_ne!(list1, list2);
    }

    #[test]
    fn partial_eq_not_same_value_test() {
        // Arrange
        let list1 = (0..5).collect::<List<_>>();
        let list2 = (1..6).collect::<List<_>>();

        // Assert
        assert_ne!(list1, list2);
    }

    #[test]
    fn fmt_test() {
        // Arrange
        let list = (0..2).collect::<List<_>>();

        // Convert the debug representation to a string
        let debug_output = format!("{:?}", list);

        // Assert
        assert_eq!("[0, 1]", debug_output);
    }

    #[test]
    fn get_index_test() {
        // Arrange
        let list = (0..2).collect::<List<_>>();

        // Assert
        assert_eq!(0, list[0]);
        assert_eq!(Some(&0), list.get(0));
        assert_eq!(1, list[1]);
        assert_eq!(Some(&1), list.get(1));
        assert_eq!(None, list.get(2));
    }

    #[test]
    fn get_mut_test() {
        // Arrange
        let mut list = (0..2).collect::<List<_>>();

        // Act & Assert
        list[0] = 1;
        assert_eq!(1, list[0]);
        list.get_mut(1).map(|v| *v += 1);
        assert_eq!(2, list[1]);
    }

    #[test]
    #[should_panic]
    fn index_panic_out_of_bounds_test() {
        // Arrange
        let list = (0..2).collect::<List<_>>();

        // Assert
        list[2];
    }

    #[test]
    #[should_panic]
    fn index_mut_panic_out_of_bounds_test() {
        // Arrange
        let mut list = (0..2).collect::<List<_>>();

        // Assert
        list[2] = 2;
    }

    #[test]
    fn index_get_from_end_test() {
        // Arrange
        let list = (0..5).collect::<List<_>>();

        // Assert
        assert_eq!(list.get(3), Some(&3));
    }

    #[test]
    fn reverse_empty_test() {
        // Arrange
        let mut list = List::<i32>::new();
        list.reverse();

        // Assert
        assert!(list.is_empty());
    }

    #[test]
    fn concatenate_left_empty_test() {
        // Arrange
        let mut list = List::<i32>::new();
        let other_list = (0..5).collect::<List<_>>();
        let clone = other_list.clone();
        list.concatenate(other_list);

        // Assert
        assert_eq!(list, clone);
    }

    #[test]
    fn concatenate_right_empty_test() {
        // Arrange
        let mut list = (0..5).collect::<List<_>>();
        let other_list = List::<i32>::new();
        let clone = list.clone();
        list.concatenate(other_list);

        // Assert
        assert_eq!(list, clone);
    }

    #[test]
    fn concatenate_test() {
        // Arrange
        let mut list = (0..3).collect::<List<_>>();
        let other_list = (3..5).collect::<List<_>>();
        list.concatenate(other_list);

        // Assert
        assert_eq!(list, (0..5).collect::<List<_>>());
    }

    #[test]
    fn sort_test() {
        // Arrange
        let mut list = (0..3).collect::<List<_>>();
        let mut list2 = (3..8).collect::<List<_>>();
        list2.reverse();
        list.concatenate(list2);

        list.sort_by(|a, b| b.cmp(a));

        // Assert
        assert!(list.iter().is_sorted_by(|a, b| a > b));
        assert_eq!(list, (0..8).rev().collect::<List<_>>());
    }
}
