use core::marker::PhantomData;

use crate::{
    entity_builder::{EntityBuilder, EntityBuilderMethods},
    query_builder::{QueryBuilder, QueryBuilderMethods},
    storage::{Identifier, Storage},
    type_map::TypeMap,
};

#[derive(Debug, Default)]
pub struct World<I: Identifier, E: Storage<Key = I, Value = TypeMap>> {
    pub(crate) entities: Entities<E>,
    pub(crate) all_tables: AllTables,
}

pub trait WorldMethods: Default {
    type Key: Identifier + 'static;
    type EntityStorage: Storage<Key = Self::Key, Value = TypeMap>;
    type ComponentStorage<T: 'static>: Storage<Key = Self::Key, Value = T> + 'static;
    type AssocEntityBuilder<'a>: EntityBuilderMethods;
    type AssocQueryBuilder<'a>: QueryBuilderMethods;

    fn new() -> Self {
        Self::default()
    }

    fn world(&self) -> &World<Self::Key, Self::EntityStorage>;

    fn world_mut(&mut self) -> &mut World<Self::Key, Self::EntityStorage>;

    fn spawn(&mut self) -> EntityBuilder<'_, Self::Key, Self::EntityStorage> {
        let id = self.world_mut().entities.0.insert(TypeMap::new());
        Self::AssocEntityBuilder::create(id, self.world_mut())
    }

    fn query(&self) -> QueryBuilder<'_, Self::Key, Self::EntityStorage> {
        Self::AssocQueryBuilder::create(self.world())
    }

    fn component_mut<C: 'static>(&mut self, entity: Self::Key) -> Option<&mut C> {
        let world = self.world_mut();
        let entity_comp_ids = world.entities.0.get(entity)?;
        let component_id = entity_comp_ids.get::<ComponentId<C, Self::Key>>()?;
        let table = world
            .all_tables
            .0
            .get_mut::<Table<C, Self::Key, Self::ComponentStorage<C>>>()?;

        table.storage.get_mut(component_id.id)
    }
}

#[derive(Debug, Default)]
pub(crate) struct Entities<S: Storage>(pub(crate) S);

#[derive(Debug, Default)]
pub(crate) struct AllTables(pub(crate) TypeMap);

#[derive(Debug)]
pub(crate) struct ComponentId<C, I: Identifier> {
    pub(crate) id: I,
    pub(crate) _phantom_data: PhantomData<C>,
}

#[derive(Debug, Default)]
pub(crate) struct Table<C, I: Identifier, T: Storage<Key = I, Value = C>> {
    pub(crate) storage: T,
    _phantom_data: PhantomData<C>,
}

impl<C, I: Identifier, T: Storage<Key = I, Value = C>> Table<C, I, T> {
    pub(crate) fn new() -> Self {
        Self {
            storage: T::default(),
            _phantom_data: PhantomData,
        }
    }
}
