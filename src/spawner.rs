use crate::prelude::*;
use bevy_rand::prelude::*;
use rand_core::Rng;

pub fn spawn_boids(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    atlas_layout: Res<SpriteSheetAtlas>,
    mut rng: Single<&mut WyRand, With<GlobalRng>>,
    boid_settings: Res<BoidSettings>,
) {
    for _ in 1..=boid_settings.count {
        let pos = Vec3::new(
            (rng.next_u32() % MAP_SIZE) as f32,
            (rng.next_u32() % MAP_SIZE) as f32,
            0.0,
        );

        commands.spawn((
            Boid {
                position: pos.xy(),
                velocity: Vec2::ZERO,
                acceleration: Vec2::new(0.0, 0.0),
            },
            SpatialEntity,
            Flock,
            Transform::from_translation(pos),
            Sprite {
                image: asset_server.load("spritesheet.png"),
                custom_size: Some(Vec2::new(0.7, 1.0)),
                texture_atlas: Some(TextureAtlas {
                    layout: atlas_layout.handle.clone(),
                    //index: 89,
                    index: 94,
                }),
                ..default()
            },
        ));
    }
}
