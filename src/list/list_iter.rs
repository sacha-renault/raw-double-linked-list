use super::list_item::ItemPtr;

pub struct ListIter<'a, T> {
    pub(crate) left: Option<ItemPtr<T>>,
    pub(crate) right: Option<ItemPtr<T>>,
    pub(crate) _phantom: std::marker::PhantomData<&'a T>,
}

impl<'a, T> Iterator for ListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.left {
            Some(ptr) => {
                // Get a reference to the current value
                let value = unsafe { &(*ptr).value };

                // Ensure left isn't equal to right
                if self.left == self.right {
                    self.left = None;
                    self.right = None;
                } else {
                    // Update current to the next node
                    self.left = unsafe { (*ptr).next };
                }

                // Return the value
                Some(value)
            }
            None => None,
        }
    }
}

impl<'a, T> DoubleEndedIterator for ListIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.right {
            Some(ptr) => {
                // Get a reference to the current value
                let value = unsafe { &(*ptr).value };

                // Ensure left isn't equal to right
                if self.left == self.right {
                    self.left = None;
                    self.right = None;
                } else {
                    // Update current to the next node
                    self.right = unsafe { (*ptr).previous };
                }

                // Return the value
                Some(value)
            }
            None => None,
        }
    }
}