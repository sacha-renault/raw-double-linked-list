use super::linked_list::List;

pub struct IntoIter<T> {
    list: List<T>
}

impl<T> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { list: self }
    }
}

// Implement Iterator for IntoIter
impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // Pop from the front of the list
        // We need to implement pop_front first
        self.list.pop_front()
    }
}