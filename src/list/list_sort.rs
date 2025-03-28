use super::list_item::{DoubleLinkedListItem, ItemPtr};

pub(crate) struct UpdateListBound<T> {
    pub(crate) new_start: ItemPtr<T>,
    pub(crate) new_end: ItemPtr<T>,
}

pub fn merge_sort_by<F, T>(start: ItemPtr<T>, len: usize, f: F) -> UpdateListBound<T>
where
    F: Fn(&T, &T) -> std::cmp::Ordering + Copy,
{
    // Base case: if the list has 0 or 1 elements, it's already sorted
    if len <= 1 || start.is_null() {
        return UpdateListBound {
            new_start: start,
            new_end: start,
        };
    }

    // Calculate the length of the first half
    let mid_len = len / 2;

    // Find the middle node and the last node of the first half
    let mut middle = start;
    for _ in 0..mid_len - 1 {
        middle = unsafe { (*middle).next.unwrap() };
    }

    // Get the first node of the second half
    let second_half = unsafe { (*middle).next.unwrap() };

    // Temporarily break the list into two separate lists
    unsafe {
        // Disconnect the two halves
        if let Some(second) = (*middle).next {
            (*second).previous = None;
        }
        (*middle).next = None;
    }

    // Sort both halves
    let left_bounds = merge_sort_by(start, mid_len, f);
    let right_bounds = merge_sort_by(second_half, len - mid_len, f);

    // Merge the sorted halves back together
    merge(
        left_bounds.new_start,
        left_bounds.new_end,
        right_bounds.new_start,
        right_bounds.new_end,
        f,
    )
}

// Helper function to merge two sorted lists
fn merge<F, T>(
    left_head: ItemPtr<T>,
    left_tail: ItemPtr<T>,
    right_head: ItemPtr<T>,
    right_tail: ItemPtr<T>,
    f: F,
) -> UpdateListBound<T>
where
    F: Fn(&T, &T) -> std::cmp::Ordering,
{
    // Handle empty list cases
    if left_head.is_null() {
        return UpdateListBound {
            new_start: right_head,
            new_end: right_tail,
        };
    }
    if right_head.is_null() {
        return UpdateListBound {
            new_start: left_head,
            new_end: left_tail,
        };
    }

    // Create a dummy head to simplify the merging logic
    let mut dummy = unsafe { std::mem::zeroed::<DoubleLinkedListItem<T>>() };
    let dummy_ptr: ItemPtr<T> = &mut dummy;
    let mut current = dummy_ptr;

    let mut left = left_head;
    let mut right = right_head;

    // Merge the two lists
    while !left.is_null() && !right.is_null() {
        let comparison = unsafe { f(&(*left).value, &(*right).value) };

        if comparison == std::cmp::Ordering::Less || comparison == std::cmp::Ordering::Equal {
            // Take node from left list
            let next_left = unsafe { (*left).next.unwrap_or(std::ptr::null_mut()) };
            unsafe {
                (*current).next = Some(left);
                (*left).previous = Some(current);
                current = left;
                left = next_left;
                (*current).next = None; // Break the original link
            }
        } else {
            // Take node from right list
            let next_right = unsafe { (*right).next.unwrap_or(std::ptr::null_mut()) };
            unsafe {
                (*current).next = Some(right);
                (*right).previous = Some(current);
                current = right;
                right = next_right;
                (*current).next = None; // Break the original link
            }
        }
    }

    // Attach remaining nodes and determine the tail
    let tail;
    if !left.is_null() {
        unsafe {
            (*current).next = Some(left);
            (*left).previous = Some(current);
        }
        tail = left_tail;
    } else if !right.is_null() {
        unsafe {
            (*current).next = Some(right);
            (*right).previous = Some(current);
        }
        tail = right_tail;
    } else {
        // Both lists are fully consumed, current is the tail
        tail = current;
    }

    // Get the head (skip the dummy node)
    let head = unsafe { (*dummy_ptr).next.unwrap_or(std::ptr::null_mut()) };
    if !head.is_null() {
        // Disconnect from dummy node
        unsafe {
            (*head).previous = None;
        }
    }

    UpdateListBound {
        new_start: head,
        new_end: tail,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_both_empty() {
        // Both lists are empty (null)
        let left_head: ItemPtr<i32> = std::ptr::null_mut();
        let left_tail: ItemPtr<i32> = std::ptr::null_mut();
        let right_head: ItemPtr<i32> = std::ptr::null_mut();
        let right_tail: ItemPtr<i32> = std::ptr::null_mut();

        let result = merge(left_head, left_tail, right_head, right_tail, |a, b| {
            a.cmp(b)
        });

        // Result should also be null
        assert!(result.new_start.is_null());
        assert!(result.new_end.is_null());
    }

    #[test]
    fn test_right_empty() {
        // Create a non-empty left list [1, 3, 5]
        let mut node1 = DoubleLinkedListItem {
            value: 1,
            next: None,
            previous: None,
        };
        let mut node2 = DoubleLinkedListItem {
            value: 3,
            next: None,
            previous: None,
        };
        let mut node3 = DoubleLinkedListItem {
            value: 5,
            next: None,
            previous: None,
        };

        // Link the nodes
        node1.next = Some(&mut node2);
        node2.previous = Some(&mut node1);
        node2.next = Some(&mut node3);
        node3.previous = Some(&mut node2);

        let left_head: ItemPtr<i32> = &mut node1;
        let left_tail: ItemPtr<i32> = &mut node3;

        // Right list is empty (null)
        let right_head: ItemPtr<i32> = std::ptr::null_mut();
        let right_tail: ItemPtr<i32> = std::ptr::null_mut();

        let result = merge(left_head, left_tail, right_head, right_tail, |a, b| {
            a.cmp(b)
        });

        // Result should be the same as left list
        assert_eq!(result.new_start, left_head);
        assert_eq!(result.new_end, left_tail);

        // Verify list integrity is maintained
        unsafe {
            // Check first node
            assert_eq!((*result.new_start).value, 1);
            assert!((*result.new_start).previous.is_none());

            // Check connections to second node
            let second = (*result.new_start).next.unwrap();
            assert_eq!((*second).value, 3);
            assert_eq!((*second).previous.unwrap(), result.new_start);

            // Check connections to third node
            let third = (*second).next.unwrap();
            assert_eq!((*third).value, 5);
            assert_eq!((*third).previous.unwrap(), second);
            assert!((*third).next.is_none());
        }
    }

    #[test]
    fn test_merge_base_cases() {
        // Create a non-empty list [2, 4, 6]
        let mut node1 = DoubleLinkedListItem {
            value: 2,
            next: None,
            previous: None,
        };
        let mut node2 = DoubleLinkedListItem {
            value: 4,
            next: None,
            previous: None,
        };
        let mut node3 = DoubleLinkedListItem {
            value: 6,
            next: None,
            previous: None,
        };

        // Link the nodes
        node1.next = Some(&mut node2);
        node2.previous = Some(&mut node1);
        node2.next = Some(&mut node3);
        node3.previous = Some(&mut node2);

        let list_head: ItemPtr<i32> = &mut node1;
        let list_tail: ItemPtr<i32> = &mut node3;

        // Empty list
        let empty: ItemPtr<i32> = std::ptr::null_mut();

        // Test case 1: Left list is empty
        let result1 = merge(empty, empty, list_head, list_tail, |a, b| a.cmp(b));

        // Result should be the right list
        assert_eq!(result1.new_start, list_head);
        assert_eq!(result1.new_end, list_tail);

        // Test case 2: Right list is empty
        let result2 = merge(list_head, list_tail, empty, empty, |a, b| a.cmp(b));

        // Result should be the left list
        assert_eq!(result2.new_start, list_head);
        assert_eq!(result2.new_end, list_tail);

        // Test case 3: Both lists are empty
        let result3 = merge(empty, empty, empty, empty, |a, b| a.cmp(b));

        // Result should be empty
        assert!(result3.new_start.is_null());
        assert!(result3.new_end.is_null());
    }
}
