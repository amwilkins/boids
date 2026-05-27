use crate::prelude::*;
use bevy_rand::prelude::*;
use rand_core::Rng;

// pub fn spawn_player(mut commands: Commands) {
//     commands.spawn((
//         Player,
//         Transform::from_translation(Vec3::new(5., 5., 100.)),
//         Sprite {
//             color: Color::srgb(0., 0.47, 1.),
//             custom_size: Some(Vec2::new(PLAYER_SIZE, PLAYER_SIZE)),
//             ..default()
//         },
//     ));
// }

pub fn spawn_boids(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    atlas_layout: Res<SpriteSheetAtlas>,
    mut rng: Single<&mut WyRand, With<GlobalRng>>,
) {
    for _ in 1..BOID_COUNT {
        let pos = Vec3::new(
            (rng.next_u32() % MAP_SIZE) as f32,
            (rng.next_u32() % MAP_SIZE) as f32,
            0.0,
        );

        commands.spawn((
            Boid {
                position: pos.xy(),
                //velocity: pos.xy().normalize_or_zero(),
                velocity: Vec2::ZERO,
                acceleration: Vec2::new(0.0, 0.0),
            },
            Flock,
            Transform::from_translation(pos),
            Sprite {
                // CHANGED to spritesheet.png.
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
