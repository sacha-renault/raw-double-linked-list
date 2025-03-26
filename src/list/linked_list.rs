use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::{Index, IndexMut};

use super::list_item::{DoubleLinkedListItem, ItemPtr};
use super::list_iter::ListIter;
use super::list_utility::{find_index_through, get_ptr_starting_point, Side};

#[derive(Default)]
pub struct List<T> {
    start: Option<ItemPtr<T>>,
    end: Option<ItemPtr<T>>,
    len: usize,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            start: None,
            end: None,
            len: 0,
        }
    }

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

    pub fn insert(&mut self, index: usize, value: T) {
        // Check if it couldn't be replace with push back or front
        if index == 0 {
            return self.push_front(value);
        } else if index == self.len {
            return self.push_back(value);
        } else if index > self.len {
            panic!("Index out of bounds");
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
        }

    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.len == 0 {
            return None; // Empty list
        }

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
        }

        // Decrement length
        self.len -= 1;

        // Convert the raw pointer back to a Box and return the value
        unsafe {
            let box_item = Box::from_raw(front_ptr);
            Some(box_item.value)
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.len == 0 {
            return None; // Empty list
        }

        // Get the front node
        let back_ptr = self.end?;

        // Update the start pointer
        self.end = unsafe { (*back_ptr).previous };

        // If there's a new start, update its previous pointer
        if let Some(new_end) = self.end {
            unsafe { (*new_end).next = None; }
        } else {
            // List is now empty
            self.end = None;
        }

        // Decrement length
        self.len -= 1;

        // Convert the raw pointer back to a Box and return the value
        unsafe {
            let box_item = Box::from_raw(back_ptr);
            Some(box_item.value)
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    /// Inplace reverse
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
            unsafe { std::mem::swap(&mut (*ptr).next, &mut (*ptr).previous); }

            // Move to the next node (which was saved before the swap)
            current = next_node;
        }
    }

    pub fn iter(&self) -> ListIter<T> {
        ListIter {
            left: self.start,
            right: self.end,
            _phantom: PhantomData,
        }
    }

    fn _get_ptr_at_index(&self, index: usize) -> Option<ItemPtr<T>> {
        // Early exit condition
        if self.len == 0 || index > self.len {
            return None;
        }

        // Start by finding where we should starting from
        let side = get_ptr_starting_point(index, self.len);

        // Get the appropriate starting pointer based on side
        let raw_ptr = match &side {
            &Side::Left => self.start?,
            &Side::Right => self.end?
        };

        // retrieve the associated node
        find_index_through(
            raw_ptr, index, self.len, &side)
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        let raw_ptr = self._get_ptr_at_index(index);
        raw_ptr.map(|ptr| unsafe { &(*ptr).value })
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        let raw_ptr = self._get_ptr_at_index(index);
        raw_ptr.map(|ptr| unsafe { &mut (*ptr).value })
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn clear(&mut self) {
        while self.pop_front().is_some() {}
    }

    pub fn first(&self) -> Option<&T> {
        self.start.map(|ptr| unsafe { &(*ptr).value })
    }

    pub fn last(&self) -> Option<&T> {
        self.end.map(|ptr| unsafe { &(*ptr).value })
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
