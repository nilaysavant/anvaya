# Anvaya

ECS _like_ dynamic storage in ~500 LOC.

> 🚧 WIP toy project. Use at your own risk ⚠️

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
