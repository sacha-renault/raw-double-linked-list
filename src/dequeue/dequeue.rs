pub trait Dequeue {
    type Item;

    fn pop(&mut self) -> Option<Self::Item>;
    fn push(&mut self, value: Self::Item);
    fn len(&self) -> usize;
}