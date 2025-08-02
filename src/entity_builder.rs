use core::marker::PhantomData;

use crate::{
    storage::{Identifier, Storage},
    type_map::TypeMap,
    world::{ComponentId, Table, World},
};

#[derive(Debug)]
pub struct EntityBuilder<'a, I: Identifier, E: Storage<Key = I, Value = TypeMap>> {
    pub(crate) id: I,
    pub(crate) world: &'a mut World<I, E>,
}

pub trait EntityBuilderMethods {
    type Key: Identifier + 'static;
    type EntityStorage: Storage<Key = Self::Key, Value = TypeMap>;
    type ComponentStorage<T: 'static>: Storage<Key = Self::Key, Value = T> + 'static;

    fn id(&self) -> Self::Key;

    fn world(&mut self) -> &mut World<Self::Key, Self::EntityStorage>;

    fn insert<C: 'static>(&mut self, component: C) -> &mut Self {
        let id = self.id();
        let world = self.world();
        let table = if let Some(table) = world.all_tables.0.get_mut::<Table<
            C,
            Self::Key,
            Self::ComponentStorage<C>,
        >>() {
            table
        } else {
            world
                .all_tables
                .0
                .insert::<Table<C, Self::Key, Self::ComponentStorage<C>>>(Table::new());
            world
                .all_tables
                .0
                .get_mut::<Table<C, Self::Key, Self::ComponentStorage<C>>>()
                .unwrap()
        };

        let comp_id = table.storage.insert(component);
        let entity = world
            .entities
            .0
            .get_mut(id)
            .unwrap_or_else(|| panic!("Entity not found for id: {id}"));

        entity.insert::<ComponentId<C, Self::Key>>(ComponentId {
            id: comp_id,
            _phantom_data: PhantomData,
        });

        self
    }
}
