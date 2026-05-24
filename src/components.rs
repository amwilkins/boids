pub use crate::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Mob;

#[derive(Component)]
pub struct ChasePlayer;

#[derive(Component, Clone, Copy)]
pub struct Walls;

// #[derive(Resource, Default, Clone, Copy, Debug, Deref, DerefMut)]
// pub struct SessionSeed(pub u64);


