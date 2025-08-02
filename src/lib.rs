#![doc = include_str!("../README.md")]

mod entity_builder;
/// Various storage integrations.
mod integrations;
mod query_builder;
mod storage;
/// Type Map data structure.
mod type_map;
mod world;

/// Common imports prelude.
pub mod prelude {
    use super::*;

    /// Entity builder exports for external impls.
    pub use entity_builder::{EntityBuilder, EntityBuilderMethods};
    /// Query builder exports for external impls.
    pub use query_builder::{EntityFrequency, QueryBuilder, QueryBuilderMethods};
    /// Storage exports for external impls.
    pub use storage::{Identifier, Storage};
    /// Export [`TypeMap`] for re-use with external storage impls.
    pub use type_map::TypeMap;
    /// Exports world, traits etc for external storage impls.
    pub use world::{World, WorldMethods};
}
