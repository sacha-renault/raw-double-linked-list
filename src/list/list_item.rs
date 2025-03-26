#[derive(Debug, Default)]
pub(crate) struct DoubleLinkedListItem<T> {
    pub(crate) value: T,
    pub(crate) next: Option<ItemPtr<T>>,
    pub(crate) previous: Option<ItemPtr<T>>,
}

pub(crate) type ItemPtr<T> = *mut DoubleLinkedListItem<T>;