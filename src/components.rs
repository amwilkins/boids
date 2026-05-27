use bevy::math::bounding::RayCast2d;

pub use crate::prelude::*;


// Default must be implemented to define this as a required component for the Wall component below
#[derive(Component, Default)]
pub struct Collider;

#[derive(Component)]
pub struct Player;

#[derive(Component, Debug)]
pub struct Boid {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    //pub eyes: Eyes,
}

#[derive(Component, Debug)]
pub struct Eyes {
        pub left: RayCast2d,
        pub right: RayCast2d,
}


// #[derive(Component)]
// pub struct ChasePlayer;

#[derive(Component)]
pub struct Flock;

#[derive(Component, Clone, Copy)]
#[require(Collider)]
pub struct Wall;

// #[derive(Component)]
// pub struct Sprite {
// }

#[derive(Resource)]
pub struct SpriteSheetAtlas {
    pub handle: Handle<TextureAtlasLayout>,
}


// #[derive(Resource, Default, Clone, Copy, Debug, Deref, DerefMut)]
// pub struct SessionSeed(pub u64);


// #[derive(Resource, Default)]
// pub struct Game {
//     pub score: i32,
// }
