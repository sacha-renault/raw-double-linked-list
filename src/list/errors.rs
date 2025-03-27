pub enum Errors {
    /// Indicates an access attempt outside the valid range of indices
    OutOfBounds,

    /// Indicates an internal inconsistency in the list structure
    InternalError,
}
