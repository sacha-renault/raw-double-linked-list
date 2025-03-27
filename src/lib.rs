mod dequeue;
mod list;

pub mod prelude {
    pub use super::dequeue::{dequeue::Dequeue, queue::Queue, stack::Stack};
    pub use super::list::linked_list::List;
}
