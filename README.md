# Anvaya

ECS [_like_](#limitations) dynamic storage in ~500 LOC.

> 🚧 [WIP](#goals) [toy project](#motivations). Use at your own risk ⚠️

```rust
use anvaya::prelude::*;

// Create Components (of any type with no boilerplate)...
struct Player(&'static str);
struct Age(u8);

// World and spawn...
let mut world = World::new();
world.spawn().insert(Player("Mike")).insert(Age(30));
world.spawn().insert(Player("Hannah")).insert(Age(25));

// Query and filter...
let mut query = world.query();
let mut results = query
    .with::<Age>()
    .get::<Player>()
    .unwrap()
    .map(|(_, player)| player.0);

// Validate...
assert_eq!(results.next().unwrap(), "Mike");
assert_eq!(results.next().unwrap(), "Hannah");
```

## Features

- Simple implementation using the `TypeMap` data structure.
- No `unsafe`, no `Clone`, no **smart pointers/atomics**. Just `Box<dyn Any>`.
- Uses [`Slab`](https://crates.io/crates/slab) as default tabular storage. But allows swapping it for your own custom storage by impl a few traits like `Storage` etc. See [`custom_storage.rs`](./examples/custom_storage.rs) example.

> The above [features](#features) are subject to change based on the [goals](#goals) of the project.

## Limitations

- Does not strictly follow an ECS architecture (with Archetypes etc).
- Currently supports only few methods and limited queries. Queries to get or mutate multiple components together are not _yet_ supported.

## Goals

- Allow more ECS like queries and mutations in the future.
- Thus follow a more ECS like architecture to achieve the above.

## Motivations

- Started as a toy project experimenting with dynamic storage in a strictly type safe language like Rust.
- With the end goal of building an ECS like data structure, there were key learnings wrt [Generic Types, Traits, and Lifetimes](https://doc.rust-lang.org/book/ch10-00-generics.html) in Rust.
- Type storage using `Box<dyn Any>` and type identifications using `TypeID` led to creation of data structures like `TypeMap`. This was the key inspiration to start this project.
