pub use crate::prelude::*;
//use bevy_spatial::kdtree::KDTree2;

// Default must be implemented to define this as a required component for the Wall component below
#[derive(Component, Default)]
pub struct Collider;

#[derive(Component)]
pub struct Player;

#[derive(Component, Debug, Clone)]
pub struct Boid {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
}


#[derive(Component)]
pub struct Flock;

#[derive(Component, Clone, Copy)]
#[require(Collider)]
pub struct Wall;

#[derive(Component)]
pub struct FpsText;

#[derive(Component, Default)]
pub struct SpatialEntity;

