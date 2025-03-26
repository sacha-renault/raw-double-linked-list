#[derive(Debug, Default)]
pub(crate) struct DoubleLinkedListItem<T> {
    pub(crate) value: T,
    pub(crate) next: Option<*mut DoubleLinkedListItem<T>>,
    pub(crate) previous: Option<*mut DoubleLinkedListItem<T>>,
}