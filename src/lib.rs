mod dequeue;
mod list;

pub mod prelude {
    pub use super::dequeue::{single_ended_collection::SingleEndedCollection , queue::Queue, stack::Stack};
    pub use super::list::linked_list::List;
}
