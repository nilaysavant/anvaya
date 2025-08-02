//! Example with Custom Storage.
//!
//! - Using a wrapper of Slab instead of slab directly.
//!

use anvaya::prelude::*;
use slab::Slab;

fn main() {
    // Create Components...
    struct Player(&'static str);
    enum Abilities {
        Shoot,
        Melee,
    }
    // World and spawn...
    let mut world = MyWorld::new();
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
    let results = query
        .with::<Abilities>() //
        .get::<Player>()
        .unwrap()
        .map(|(_, p)| p.0)
        .collect::<Vec<_>>();
    // Validate...
    assert_eq!(results[0], "Mike");
    assert_eq!(results[1], "Hannah");
    dbg!(results);
}

#[derive(Debug)]
struct MyStorage<T>(Slab<T>);

impl<T> Default for MyStorage<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T> Storage for MyStorage<T> {
    type Key = usize;

    type Value = T;

    fn new() -> Self {
        Self(Slab::new())
    }

    fn with_capacity(capacity: usize) -> Self {
        Self(Slab::with_capacity(capacity))
    }

    fn insert(&mut self, val: Self::Value) -> Self::Key {
        self.0.insert(val)
    }

    fn get(&self, key: Self::Key) -> Option<&Self::Value> {
        self.0.get(key)
    }

    fn get_mut(&mut self, key: Self::Key) -> Option<&mut Self::Value> {
        self.0.get_mut(key)
    }

    fn key_of(&self, val: &Self::Value) -> Self::Key {
        self.0.key_of(val)
    }

    fn has(&self, key: Self::Key) -> bool {
        self.0.contains(key)
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn iter(&self) -> impl Iterator<Item = (Self::Key, &Self::Value)> {
        self.0.iter()
    }
}

#[derive(Debug, Default)]
struct MyWorld(World<usize, MyStorage<TypeMap>>);

impl WorldMethods for MyWorld {
    type Key = usize;

    type EntityStorage = MyStorage<TypeMap>;

    type ComponentStorage<T: 'static> = MyStorage<T>;

    type AssocEntityBuilder<'a> = MyEntityBuilder<'a>;

    type AssocQueryBuilder<'a> = MyQueryBuilder<'a>;

    fn world(&self) -> &World<Self::Key, Self::EntityStorage> {
        &self.0
    }

    fn world_mut(&mut self) -> &mut World<Self::Key, Self::EntityStorage> {
        &mut self.0
    }
}

struct MyEntityBuilder<'a>(EntityBuilder<'a, usize, MyStorage<TypeMap>>);

impl<'a> EntityBuilderMethods<'a> for MyEntityBuilder<'a> {
    type Key = usize;

    type EntityStorage = MyStorage<TypeMap>;

    type ComponentStorage<T: 'static> = MyStorage<T>;

    fn create(id: Self::Key, world: &'a mut World<Self::Key, Self::EntityStorage>) -> Self {
        Self(EntityBuilder::new(id, world))
    }

    fn id(&self) -> Self::Key {
        self.0.id
    }

    fn world(&mut self) -> &mut World<Self::Key, Self::EntityStorage> {
        self.0.world
    }
}

struct MyQueryBuilder<'a>(QueryBuilder<'a, usize, MyStorage<TypeMap>>);

impl<'a> QueryBuilderMethods<'a> for MyQueryBuilder<'a> {
    type Key = usize;

    type EntityStorage = MyStorage<TypeMap>;

    type ComponentStorage<T: 'static> = MyStorage<T>;

    fn create(world: &'a World<Self::Key, Self::EntityStorage>) -> Self {
        Self(QueryBuilder::new(world))
    }

    fn with_call_count(&mut self) -> &mut u32 {
        &mut self.0.with_call_count
    }

    fn entity_freq(&self) -> &EntityFrequency<Self::Key> {
        &self.0.entity_freq
    }

    fn entity_freq_mut(&mut self) -> &mut EntityFrequency<Self::Key> {
        &mut self.0.entity_freq
    }

    fn world(&self) -> &World<Self::Key, Self::EntityStorage> {
        self.0.world
    }
}
