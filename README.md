# Rust Double Linked List

A raw double linked list implementation in Rust.

## Overview

This crate provides a double linked list implementation in Rust. While functional, it comes with important caveats that you should consider before using it in your project.

### Performance Considerations

- In nearly all cases, `Vec` is a superior choice due to its contiguous memory layout which provides better CPU cache utilization
- While this implementation offers O(1) non-amortized push and pop operations, this alone rarely justifies its use
- Element access is **not** O(1), unlike array-based structures
- For stack and queue use cases, Rust's `VecDeque` provides excellent performance characteristics with a more thoroughly tested implementation

### When You Might Consider This Implementation

Only very specific edge cases might benefit from a double linked list, such as:
- When you need guaranteed O(1) **NON AMORTIZED** insertion/removal at both ends
- When you need stable references to elements that won't be invalidated by other operations
- When the overhead of moving large elements is problematic (though `Box<T>` with vector-based containers often addresses this concern)

### Alternative Approaches

If you're considering this implementation, you might want to:
1. Reconsider your design to better align with vector-based data structures
2. Use Rust's standard library collections like `Vec` or `VecDeque`

## Honest Assessment

Why did I do it if I do not recommand to use it ? Why not ...

## Usage

Despite the warnings, if you still wish to use this implementation:

### List

```rust
use raw_double_linked_list::prelude::*;

fn main() {
    // Create a new empty list
    let mut list = List::new();

    // Add elements to the list
    list.push_back(10);
    list.push_back(20);
    list.push_front(5);

    println!("List: {:?}", list);  // Output: List: [5, 10, 20]
    println!("Length: {}", list.len());  // Output: Length: 3

    // Access elements
    println!("First element: {:?}", list.first());  // Output: First element: Some(5)
    println!("Last element: {:?}", list.last());    // Output: Last element: Some(20)
    println!("Element at index 1: {:?}", list.get(1));  // Output: Element at index 1: Some(10)

    // Modify an element
    if let Some(elem) = list.get_mut(1) {
        *elem = 15;
    }
    println!("After modification: {:?}", list);  // Output: After modification: [5, 15, 20]

    // Insert at specific position
    list.insert(2, 18).unwrap();
    println!("After insertion: {:?}", list);  // Output: After insertion: [5, 15, 18, 20]

    // Remove elements
    let first = list.pop_front();
    let last = list.pop_back();
    println!("Removed first: {:?}, Removed last: {:?}", first, last);  // Output: Removed first: Some(5), Removed last: Some(20)
    println!("After removal: {:?}", list);  // Output: After removal: [15, 18]

    // Reverse the list
    list.reverse();
    println!("After reversal: {:?}", list);  // Output: After reversal: [18, 15]

    // Create a list from an iterator
    let vec = vec![1, 2, 3, 4];
    let list_from_iter = List::from_iter(vec);
    println!("List from iterator: {:?}", list_from_iter);  // Output: List from iterator: [1, 2, 3, 4]

    // Iterate through elements
    println!("Iterating through list:");
    for item in list_from_iter.iter() {
        println!("  {}", item);
    }

    // Check if empty and clear
    println!("Is empty: {}", list.is_empty());  // Output: Is empty: false
    list.clear();
    println!("After clearing, is empty: {}", list.is_empty());  // Output: After clearing, is empty: true
}
```

### Queue

```rust
// Create a new queue
let mut queue = Queue::new();

// Push elements to the queue
queue.push(10);
queue.push(20);
queue.push(30);

println!("Queue length: {}", queue.len());  // Output: Queue length: 3

// Pop elements from the queue (FIFO: First In, First Out)
println!("Popped: {:?}", queue.pop());  // Output: Popped: Some(10)
println!("Popped: {:?}", queue.pop());  // Output: Popped: Some(20)
println!("Popped: {:?}", queue.pop());  // Output: Popped: Some(30)
println!("Popped: {:?}", queue.pop());  // Output: Popped: None

println!("Is queue empty? {}", queue.is_empty());  // Output: Is queue empty? true
```

### Stack

```rust
// Create a new stack
let mut stack = Stack::new();

// Push elements onto the stack
stack.push(10);
stack.push(20);
stack.push(30);

println!("Stack length: {}", stack.len());  // Output: Stack length: 3

// Pop elements from the stack (LIFO: Last In, First Out)
println!("Popped: {:?}", stack.pop());  // Output: Popped: Some(30)
println!("Popped: {:?}", stack.pop());  // Output: Popped: Some(20)
println!("Popped: {:?}", stack.pop());  // Output: Popped: Some(10)
println!("Popped: {:?}", stack.pop());  // Output: Popped: None

println!("Is stack empty? {}", stack.is_empty());  // Output: Is stack empty? true
```

## Installation

### Adding as a Git Dependency

To use this library in your project without going through crates.io, add the following to your `Cargo.toml` file:

```toml
[dependencies]
raw-double-linked-list = { git = "https://github.com/sacha-renault/raw-double-linked-list" }
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.