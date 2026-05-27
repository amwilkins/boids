use bevy::{prelude::*, window::WindowResolution};
use bevy_rand::prelude::*;

mod components;
mod input;
mod map;
mod spawner;
mod systems;

mod prelude {
    pub use crate::components::*;
    //pub use crate::input::*;
    pub use crate::map::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use bevy::prelude::*;
    pub const MAP_SIZE: u32 = 50;
    pub const GRID_WIDTH: f32 = 0.05;
    //pub const PLAYER_SIZE: f32 = 0.7;
    pub const BOID_COUNT: u32 = 50;
    pub const BOID_MAX_SPEED: f32 = 0.05;
}

use prelude::*;

fn main() {
    App::new()
        // sharp sprites
        //.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        // window setup
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from("Boids"),
                // fill the entire browser window
                fit_canvas_to_parent: true,
                // don't hijack keyboard shortcuts like F5, F6, F12, Ctrl+R etc.
                prevent_default_event_handling: false,
                resolution: WindowResolution::new(960, 540).with_scale_factor_override(16.),
                mode: bevy::window::WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        }))
        // randomness
        .add_plugins(EntropyPlugin::<WyRand>::with_seed(42u64.to_ne_bytes()))
        .init_resource::<SpriteSheetAtlas>()
        // background color
        .insert_resource(ClearColor(Color::srgb(0.2, 0.2, 0.2)))
        .add_systems(
            Startup,
            (
                setup,
                generate_map,
                spawn_boids,
                //spawn_player,
                //spawn_player.after(generate_map),
                //spawn_mob.after(generate_map),
            ),
        )
        //.add_systems(Update, flock::show_boid_eyes)
        .add_systems(
            FixedUpdate,
            (
                //move_player,
                camera_follow,
                flock::flock,
                //chaseplayer::chase_player
            ),
        )
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn((
        Camera2d::default(),
        Transform::from_xyz(MAP_SIZE as f32 / 2.0, MAP_SIZE as f32 / 2.0, 100.0),
    ));

    // Horizontal lines
    for i in 0..=MAP_SIZE {
        commands.spawn((
            //Transform::from_translation(Vec3::new(0., i as f32 - MAP_SIZE as f32 / 2., 0.)),
            Transform::from_translation(Vec3::new(MAP_SIZE as f32 / 2., i as f32, 0.)),
            Sprite {
                color: Color::srgb(0.27, 0.27, 0.27),
                custom_size: Some(Vec2::new(MAP_SIZE as f32, GRID_WIDTH)),
                ..default()
            },
        ));
    }

    // Vertical lines
    for i in 0..=MAP_SIZE {
        commands.spawn((
            //Transform::from_translation(Vec3::new(i as f32 - MAP_SIZE as f32 / 2., 0., 0.)),
            Transform::from_translation(Vec3::new(i as f32, MAP_SIZE as f32 / 2., 0.)),
            Sprite {
                color: Color::srgb(0.27, 0.27, 0.27),
                custom_size: Some(Vec2::new(GRID_WIDTH, MAP_SIZE as f32)),
                ..default()
            },
        ));
    }
}

fn camera_follow(
    player: Query<(&Player, &Transform)>,
    mut camera: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    for (_player, player_transofrm) in &player {
        let pos = player_transofrm.translation;
        for mut transform in &mut camera {
            transform.translation.x = pos.x;
            transform.translation.y = pos.y;
        }
    }
}

/// An initialization of the sprite sheet atlas, ran from `init_resource`.
impl FromWorld for SpriteSheetAtlas {
    fn from_world(world: &mut World) -> Self {
        // The spritesheet is composed of 16x16 squares.
        // There are 8 sprite columns, spread across 1 row.
        // There is no padding between the cells (None) and no offset (None)
        let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 160, 2, None, None);
        // Grab the active atlases stored by Bevy.
        let mut texture_atlases = world
            .get_resource_mut::<Assets<TextureAtlasLayout>>()
            .unwrap();
        // Add the new Atlas in Bevy's atlases and store it in the Resource.
        Self {
            handle: texture_atlases.add(layout),
        }
    }
}
