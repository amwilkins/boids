use bevy::{camera::ScalingMode, math::bounding::*, prelude::*};
use bevy_rand::prelude::EntropyPlugin;
use bevy_rand::prelude::WyRand;
//use bevy_rand::prelude::{ForkableSeed, GlobalRng};
//use rand_core::Rng;
use console_log;

mod components;
mod input;
mod map;
mod spawner;
mod systems;

mod prelude {
    pub use crate::components::*;
    pub use crate::input::*;
    pub use crate::map::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use bevy::prelude::*;
    pub const MAP_SIZE: u32 = 50;
    pub const GRID_WIDTH: f32 = 0.05;
    pub const PLAYER_SIZE: f32 = 0.7;
    pub const MOB_COUNT: u32 = 10;
}

use prelude::*;


fn main() {
    let _log = console_log::init();
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                // fill the entire browser window
                fit_canvas_to_parent: true,
                // don't hijack keyboard shortcuts like F5, F6, F12, Ctrl+R etc.
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EntropyPlugin::<WyRand>::default())
        .init_resource::<Game>()
        .insert_resource(ClearColor(Color::srgb(0.53, 0.53, 0.53)))
        .add_systems(
            Startup,
            (
                setup,
                generate_map,
                spawn_mob,
                spawn_player,
                //spawn_player.after(generate_map),
                              //spawn_mob.after(generate_map),
            ),
        )
        .add_systems(Update, (move_player, camera_follow, chaseplayer::chase_player))
        //.add_systems(PostUpdate, collisions::resolve_collisions)
        .run();
}


// #[derive(Component)]
// struct Source;

fn setup(mut commands: Commands, mut game: ResMut<Game>) {
    //, mut global: Single<&mut WyRand, With<GlobalRng>>) {
    // set rand seed
    //commands.spawn((Source, global.fork_seed()));

    game.score = 0;

    // Camera
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::AutoMax {
                max_width: 32.0,
                max_height: 18.0,
            },
            ..OrthographicProjection::default_2d()
        }),
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

// // update the score displayed during the game
// fn scoreboard_system(game: Res<Game>, mut display: Single<&mut Text>) {
//     display.0 = format!("Player: {}", game.pos);
// }

