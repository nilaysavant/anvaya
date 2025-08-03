//! Basic Example with Slab Storage Integration.

use anvaya::prelude::*;

fn main() {
    #[derive(Debug)]
    struct Player(&'static str);
    #[derive(Debug)]
    #[allow(dead_code)]
    struct Age(u8);
    #[derive(Debug)]
    enum Abilities {
        Shoot,
        Melee,
    }

    let mut world = World::new();
    world
        .spawn()
        .insert(Player("Mike"))
        .insert(Age(25))
        .insert(Abilities::Shoot);
    world
        .spawn()
        .insert(Player("Hannah"))
        .insert(Age(28))
        .insert(Abilities::Melee);

    let mut query = world.query();
    let Some(results) = query
        .with::<Player>()
        .with::<Age>()
        .get::<Player>()
        .map(|results| results.collect::<Vec<_>>())
    else {
        return;
    };
    let players = results
        .iter()
        .map(|(_, player)| player.0)
        .collect::<Vec<_>>();
    assert_eq!(players.len(), 2);
    assert_eq!(players[0], "Mike");
    assert_eq!(players[1], "Hannah");
    dbg!(&results);

    let Some(player1) = world.component_mut::<Player>(results[0].0) else {
        return;
    };
    assert_eq!(player1.0, "Mike");
    player1.0 = "MikeEdited";
    let mut query = world.query();
    let Some(results) = query
        .with::<Player>()
        .with::<Age>()
        .get::<Player>()
        .map(|results| results.collect::<Vec<_>>())
    else {
        return;
    };
    assert_eq!(results[0].1.0, "MikeEdited");
}
