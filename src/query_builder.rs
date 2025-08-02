use std::collections::{HashMap, HashSet};

use crate::{
    storage::{Identifier, Storage},
    type_map::TypeMap,
    world::{ComponentId, Table, World},
};

#[derive(Debug)]
pub struct QueryBuilder<'a, I: Identifier, E: Storage<Key = I, Value = TypeMap>> {
    pub(crate) with_call_count: u32,
    pub(crate) entity_freq: EntityFrequency<I>,
    pub(crate) world: &'a World<I, E>,
}

impl<'a, I: Identifier, E: Storage<Key = I, Value = TypeMap>> QueryBuilder<'a, I, E> {
    pub fn new(world: &'a World<I, E>) -> Self {
        Self {
            world,
            with_call_count: 0,
            entity_freq: EntityFrequency::new(),
        }
    }
}

pub trait QueryBuilderMethods {
    type Key: Identifier + 'static;
    type EntityStorage: Storage<Key = Self::Key, Value = TypeMap>;
    type ComponentStorage<T: 'static>: Storage<Key = Self::Key, Value = T> + 'static;

    fn create<I: Identifier + 'static, E: Storage<Key = I, Value = TypeMap>>(
        world: &World<I, E>,
    ) -> QueryBuilder<'_, I, E>;

    fn with_call_count(&mut self) -> &mut u32;

    fn entity_freq(&self) -> &EntityFrequency<Self::Key>;

    fn entity_freq_mut(&mut self) -> &mut EntityFrequency<Self::Key>;

    fn world(&self) -> &World<Self::Key, Self::EntityStorage>;

    fn with<C: 'static>(&mut self) -> &mut Self {
        let world = self.world();
        let Some(table) = world
            .all_tables
            .0
            .get::<Table<C, Self::Key, Self::ComponentStorage<C>>>()
        else {
            return self;
        };

        let entities_iter = world
            .entities
            .0
            .iter()
            .filter_map(|(entity_id, component_ids)| {
                let comp_id = component_ids.get::<ComponentId<C, Self::Key>>()?;
                Some((entity_id, comp_id))
            });

        let mut entity_ids = Vec::new();
        for (entity_id, comp_id) in entities_iter {
            let Some(_component) = table.storage.get(comp_id.id) else {
                continue;
            };
            // Collect ids into temp vec for freq analysis...
            entity_ids.push(entity_id);
        }
        // Add collected ids into freq tracker...
        entity_ids
            .iter()
            .for_each(|entity_id| self.entity_freq_mut().add_entity(*entity_id));

        *self.with_call_count() += 1;

        self
    }

    fn get<C: 'static>(&mut self) -> Option<impl Iterator<Item = (Self::Key, &C)>> {
        let with_call_count = *self.with_call_count();
        let world = self.world();
        let table = world
            .all_tables
            .0
            .get::<Table<C, Self::Key, Self::ComponentStorage<C>>>()?;
        let filtered_entities = self
            .entity_freq()
            .freq
            .iter()
            .filter_map(|(id, freq)| {
                if *freq == with_call_count {
                    Some(*id)
                } else {
                    None
                }
            })
            .collect::<HashSet<Self::Key>>();

        let entities_with_comp_ids =
            world
                .entities
                .0
                .iter()
                .filter_map(move |(entity_id, component_ids)| {
                    let comp_id = component_ids.get::<ComponentId<C, Self::Key>>()?;
                    if filtered_entities.contains(&entity_id) {
                        Some((entity_id, comp_id))
                    } else {
                        None
                    }
                });
        let entities_with_comp_values =
            entities_with_comp_ids.filter_map(|(entity_id, comp_id)| {
                let component = table.storage.get(comp_id.id)?;
                Some((entity_id, component))
            });
        Some(entities_with_comp_values)
    }
}

#[derive(Debug, Default)]
pub struct EntityFrequency<I: Identifier> {
    pub(crate) freq: HashMap<I, u32>,
}

impl<I: Identifier> EntityFrequency<I> {
    pub fn new() -> Self {
        Self {
            freq: HashMap::new(),
        }
    }

    pub fn add_entity(&mut self, id: I) {
        let freq = if let Some(freq) = self.freq.get_mut(&id) {
            freq
        } else {
            self.freq.insert(id, 0);
            self.freq.get_mut(&id).unwrap()
        };
        *freq += 1;
    }
}
