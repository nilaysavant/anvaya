use slab::Slab;

use crate::{
    entity_builder::{EntityBuilder, EntityBuilderMethods},
    query_builder::{EntityFrequency, QueryBuilder, QueryBuilderMethods},
    storage::Storage,
    type_map::TypeMap,
    world::{World, WorldMethods},
};

impl<T> Storage for Slab<T> {
    type Key = usize;

    type Value = T;

    fn new() -> Self {
        Self::new()
    }

    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }

    fn insert(&mut self, val: Self::Value) -> Self::Key {
        self.insert(val)
    }

    fn get(&self, key: Self::Key) -> Option<&Self::Value> {
        self.get(key)
    }

    fn get_mut(&mut self, key: Self::Key) -> Option<&mut Self::Value> {
        self.get_mut(key)
    }

    fn key_of(&self, val: &Self::Value) -> Self::Key {
        self.key_of(val)
    }

    fn has(&self, key: Self::Key) -> bool {
        self.contains(key)
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn iter(&self) -> impl Iterator<Item = (Self::Key, &Self::Value)> {
        self.iter()
    }
}

impl WorldMethods for World<usize, Slab<TypeMap>> {
    type Key = usize;

    type EntityStorage = Slab<TypeMap>;

    type ComponentStorage<T: 'static> = Slab<T>;

    type AssocEntityBuilder<'a> = EntityBuilder<'a, Self::Key, Self::EntityStorage>;

    type AssocQueryBuilder<'a> = QueryBuilder<'a, Self::Key, Self::EntityStorage>;

    fn world(&self) -> &World<Self::Key, Self::EntityStorage> {
        self
    }

    fn world_mut(&mut self) -> &mut World<Self::Key, Self::EntityStorage> {
        self
    }
}

impl<'a> EntityBuilderMethods<'a> for EntityBuilder<'a, usize, Slab<TypeMap>> {
    type Key = usize;

    type EntityStorage = Slab<TypeMap>;

    type ComponentStorage<T: 'static> = Slab<T>;

    fn create(id: Self::Key, world: &'a mut World<Self::Key, Self::EntityStorage>) -> Self {
        Self::new(id, world)
    }

    fn id(&self) -> Self::Key {
        self.id
    }

    fn world(&mut self) -> &mut World<Self::Key, Self::EntityStorage> {
        self.world
    }
}

impl<'a> QueryBuilderMethods<'a> for QueryBuilder<'a, usize, Slab<TypeMap>> {
    type Key = usize;

    type EntityStorage = Slab<TypeMap>;

    type ComponentStorage<T: 'static> = Slab<T>;

    fn create(world: &'a World<Self::Key, Self::EntityStorage>) -> Self {
        Self::new(world)
    }

    fn with_call_count(&mut self) -> &mut u32 {
        &mut self.with_call_count
    }

    fn entity_freq(&self) -> &EntityFrequency<Self::Key> {
        &self.entity_freq
    }

    fn entity_freq_mut(&mut self) -> &mut EntityFrequency<Self::Key> {
        &mut self.entity_freq
    }

    fn world(&self) -> &World<Self::Key, Self::EntityStorage> {
        self.world
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        // Create Components...
        struct Player(&'static str);
        enum Abilities {
            Shoot,
            Melee,
        }
        // World and spawn...
        let mut world = World::new();
        world
            .spawn()
            .insert(Player("Mike"))
            .insert(Abilities::Shoot);
        world
            .spawn()
            .insert(Player("Hannah"))
            .insert(Abilities::Melee);
        // Query and filter...
        let mut query = world.query();
        let mut results = query
            .with::<Abilities>() //
            .get::<Player>()
            .unwrap();
        // Validate...
        assert_eq!(results.next().unwrap().1.0, "Mike");
        assert_eq!(results.next().unwrap().1.0, "Hannah");
    }
}
