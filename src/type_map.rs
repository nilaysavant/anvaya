use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

/// Map types to its associated values.
///
/// - Internal Data Structure for dynamic mapping of Types to its values,
///   which makes this library possible.
/// - __Note: The Value should be of the same type as its key(type).__
/// - Inspired from __Type Driven API Design__ [Book](https://willcrichton.net/rust-api-type-patterns/registries.html#mapping-types-to-values).
///
/// # Examples
///
/// ## Primitive Types
///
/// Using primitive data types like `i32`, `f32`, `u8`, `String` etc.
///
/// ```
/// # use anvaya::prelude::TypeMap;
/// let mut type_map = TypeMap::new();
/// type_map.insert::<i32>(1);
/// type_map.insert::<String>("Hello".to_string());
/// assert_eq!(type_map.get::<i32>(), Some(&1));
/// assert_eq!(type_map.get::<String>(), Some(&"Hello".to_string()));
/// ```
///
/// ## Custom Types
///
/// Using your own predefined data types.
///
/// ```
/// # use anvaya::prelude::TypeMap;
/// #[derive(Debug, PartialEq)]
/// struct Player(String);
/// #[derive(Debug, PartialEq)]
/// struct HP(u8);
/// #[derive(Debug, PartialEq)]
/// enum Ability {
///     Melee,
///     Shoot,
/// }
/// let mut type_map = TypeMap::new();
/// type_map.insert(Player("Mike".to_string()));
/// type_map.insert(HP(100));
/// type_map.insert(Ability::Melee);
/// assert_eq!(type_map.len(), 3);
/// assert_eq!(type_map.get::<HP>(), Some(&HP(100)));
/// ```
///
#[derive(Debug, Default)]
pub struct TypeMap(HashMap<TypeId, Box<dyn Any>>);

impl TypeMap {
    /// Create a new empty [`TypeMap`].
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Create [`TypeMap`] with pre-allocated capacity.
    ///
    /// More than the capacity is allowed, but will re-allocate when that happens.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(HashMap::with_capacity(capacity))
    }

    /// Insert a value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anvaya::prelude::TypeMap;
    /// let mut type_map = TypeMap::new();
    /// type_map.insert::<i32>(1); // Prefer specifying the type as generic to prevent wrong type inference.
    /// ```
    pub fn insert<T: Any + 'static>(&mut self, t: T) {
        self.0.insert(TypeId::of::<T>(), Box::new(t));
    }

    /// Returns `true` if the [`TypeMap`] _has_ the given type.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anvaya::prelude::TypeMap;
    /// # let mut type_map = TypeMap::new();
    /// type_map.insert::<i32>(1);
    /// assert_eq!(type_map.has::<i32>(), true);
    /// ```
    pub fn has<T: Any + 'static>(&self) -> bool {
        self.0.contains_key(&TypeId::of::<T>())
    }

    /// Length of the [`TypeMap`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use anvaya::prelude::TypeMap;
    /// let type_map = TypeMap::new();
    /// assert_eq!(type_map.len(), 0);
    /// ```
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// If [`TypeMap`] is empty (length = 0).
    ///
    /// # Examples
    ///
    /// ```
    /// # use anvaya::prelude::TypeMap;
    /// let type_map = TypeMap::new();
    /// assert_eq!(type_map.is_empty(), true);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Get ref to stored value for a given type.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anvaya::prelude::TypeMap;
    /// let mut type_map = TypeMap::new();
    /// type_map.insert::<i32>(1);
    /// assert_eq!(type_map.get::<i32>(), Some(&1));
    /// ```
    pub fn get<T: Any + 'static>(&self) -> Option<&T> {
        self.0
            .get(&TypeId::of::<T>())
            .map(|t| t.downcast_ref::<T>().unwrap())
    }

    /// Get (mutable) ref to stored value for a given type.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anvaya::prelude::TypeMap;
    /// let mut type_map = TypeMap::new();
    /// type_map.insert::<i32>(1);
    /// let num = type_map.get_mut::<i32>().unwrap();
    /// *num = 2;
    /// assert_eq!(type_map.get::<i32>(), Some(&2));
    /// ```
    pub fn get_mut<T: Any + 'static>(&mut self) -> Option<&mut T> {
        self.0
            .get_mut(&TypeId::of::<T>())
            .map(|t| t.downcast_mut::<T>().unwrap())
    }

    /// Clear the [`TypeMap`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use anvaya::prelude::TypeMap;
    /// let mut type_map = TypeMap::new();
    /// type_map.insert::<i32>(1);
    /// assert_eq!(type_map.len(), 1);
    /// type_map.clear();
    /// assert_eq!(type_map.len(), 0);
    /// assert_eq!(type_map.is_empty(), true);
    /// ```
    pub fn clear(&mut self) {
        self.0.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn type_map_custom_types_basic() {
        // Using Custom types...
        #[derive(Debug, PartialEq)]
        struct Player(String);
        #[derive(Debug, PartialEq)]
        struct HP(u8);
        #[derive(Debug, PartialEq)]
        enum Ability {
            Melee,
            Shoot,
        }

        // Inserts...
        let mut type_map = TypeMap::new();
        type_map.insert(Player("Mike".to_string()));
        type_map.insert(HP(100));
        type_map.insert(Ability::Melee);
        assert_eq!(type_map.len(), 3);

        // Has Query...
        assert!(type_map.has::<Player>());
        assert!(type_map.has::<HP>());
        assert!(!type_map.has::<f32>());

        // Getters...
        assert_eq!(type_map.get::<Player>(), Some(&Player("Mike".to_string())));
        assert_eq!(type_map.get::<HP>(), Some(&HP(100)));
        assert_eq!(type_map.get::<Ability>(), Some(&Ability::Melee));

        // Mutations...
        let player = type_map.get_mut::<Player>().unwrap();
        player.0 = "Hannah".to_string();
        let hp = type_map.get_mut::<HP>().unwrap();
        hp.0 = 50;
        let ability = type_map.get_mut::<Ability>().unwrap();
        *ability = Ability::Shoot;
        assert_eq!(type_map.get::<Player>(), Some(&Player("Hannah".to_string())));
        assert_eq!(type_map.get::<HP>(), Some(&HP(50)));
        assert_eq!(type_map.get::<Ability>(), Some(&Ability::Shoot));

        // Length
        assert_eq!(type_map.len(), 3);

        // Clear...
        type_map.clear();
        assert!(type_map.is_empty());
        assert_eq!(type_map.len(), 0);
    }
}
