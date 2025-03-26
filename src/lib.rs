mod list;
mod dequeue;

pub mod prelude {
    pub use super::list::linked_list::List;
    pub use super::dequeue::{stack::Stack, queue::Queue, dequeue::Dequeue};
}
