pub use crate::prelude::*;


// Default must be implemented to define this as a required component for the Wall component below
#[derive(Component, Default)]
pub struct Collider;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Mob;

#[derive(Component)]
pub struct ChasePlayer;

// #[derive(Component)]
// pub struct Flock;

#[derive(Component, Clone, Copy)]
#[require(Collider)]
pub struct Wall;

// #[derive(Resource, Default, Clone, Copy, Debug, Deref, DerefMut)]
// pub struct SessionSeed(pub u64);


#[derive(Resource, Default)]
pub struct Game {
    pub score: i32,
}
