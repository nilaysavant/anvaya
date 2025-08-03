//! # Simple Sports Example
//!
//! - Players are filtered based on what they play. ie. (football or cricket).
//! - Also filtered if they only meet the min age criteria.

use anvaya::prelude::*;

fn main() {
    struct Player(&'static str);
    struct Age(u8);
    struct Cricketeer;
    struct Footballer;

    let mut world = World::new();
    world
        .spawn()
        .insert(Player("Mike"))
        .insert(Age(15))
        .insert(Cricketeer)
        .insert(Footballer);
    world
        .spawn()
        .insert(Player("Rahul"))
        .insert(Age(18))
        .insert(Cricketeer)
        .insert(Footballer);
    world
        .spawn()
        .insert(Player("Sam"))
        .insert(Age(22))
        .insert(Cricketeer);

    let mut query = world.query();
    let footballers_allowed = query
        .with::<Footballer>()
        .get::<Age>()
        .map(|results| {
            results
                .filter(|(_, age)| age.0 >= 16)
                .map(|(e, _)| e)
                .collect::<Vec<_>>()
        })
        .unwrap();
    let mut query = world.query();
    let cricketers_allowed = query
        .with::<Cricketeer>()
        .get::<Age>()
        .map(|results| {
            results
                .filter(|(_, age)| age.0 >= 15)
                .map(|(e, _)| e)
                .collect::<Vec<_>>()
        })
        .unwrap();

    let mut footballers = vec![];
    for entity in footballers_allowed.iter() {
        let player = world.component_mut::<Player>(*entity).unwrap();
        footballers.push(player.0);
    }

    let mut cricketers = vec![];
    for entity in cricketers_allowed.iter() {
        let player = world.component_mut::<Player>(*entity).unwrap();
        cricketers.push(player.0);
    }

    dbg!(&footballers, &cricketers);

    assert_eq!(footballers, vec!["Rahul"]);
    assert_eq!(cricketers, vec!["Mike", "Rahul", "Sam"]);
}
