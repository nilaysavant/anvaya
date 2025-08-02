use core::{
    fmt::{Debug, Display},
    hash::Hash,
};

/// Trait for ECS Storage data structure.
pub trait Storage: Default {
    /// Key Id type used to lookup the values.
    type Key: Identifier;
    /// Type for values.
    type Value;

    /// Create new storage.
    fn new() -> Self;

    /// Create with pre-allocated capacity.
    ///
    /// Should be able to expand on demand.
    fn with_capacity(capacity: usize) -> Self;

    /// Insert a [`Storage::Value`] and return a lookup [`Storage::Key`].
    fn insert(&mut self, val: Self::Value) -> Self::Key;

    /// Get [`Storage::Value`] ref using given [`Storage::Key`].
    fn get(&self, key: Self::Key) -> Option<&Self::Value>;

    /// Get [`Storage::Value`] mutable ref using given [`Storage::Key`].
    fn get_mut(&mut self, key: Self::Key) -> Option<&mut Self::Value>;

    /// Get [`Storage::Key`] of given [`Storage::Value`] ref.
    fn key_of(&self, val: &Self::Value) -> Self::Key;

    /// Check if _has_ [`Storage::Value`] for passed [`Storage::Key`].
    fn has(&self, key: Self::Key) -> bool;

    /// Length of the [`Storage`].
    fn len(&self) -> usize;

    /// Check if [`Storage`] is empty.
    fn is_empty(&self) -> bool;

    /// Iterate over `(key, &value)` pairs of [`Storage`].
    fn iter(&self) -> impl Iterator<Item = (Self::Key, &Self::Value)>;
}

/// Trait for identifier keys of [`Storage`].
pub trait Identifier: Copy + Debug + Display + Hash + Eq + PartialEq + Default {}

/// Blanket impl to allow all type impl the following to automatically impl [`Identifier`].
impl<T> Identifier for T where T: Copy + Debug + Display + Hash + Eq + PartialEq + Default {}
