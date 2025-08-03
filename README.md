# Anvaya

ECS _like_ dynamic storage in ~500 LOC.

```rust
use anvaya::prelude::*;
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
    .unwrap()
    .map(|(_, player)| player.0);
// Validate...
assert_eq!(results.next().unwrap(), "Mike");
assert_eq!(results.next().unwrap(), "Hannah");
```
