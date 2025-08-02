#![doc = include_str!("../README.md")]

/// Type Map data structure.
mod type_map;

/// Common imports prelude.
pub mod prelude {
    use super::*;

    pub use type_map::TypeMap;
}
